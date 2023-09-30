use std::f32::consts::PI;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    sprite::MaterialMesh2dBundle,
};

use crate::{assets::Skeletons, prelude::*};

// Define a plugin for the player.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add the player to the world.
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    player_camera_system,
                    player_movement_system,
                    player_laser_aim_system.run_if(in_state(GameState::Playing)),
                    player_laser_fire_system.run_if(in_state(GameState::Playing)),
                    player_animate_system,
                    player_jet_animation_system,
                ),
            );
    }
}

// Define a component for the player.
#[derive(Component)]
pub struct Player {
    // Values for animation, set by player controls.
    pub thrust: f32,
    pub side_braking: f32,
    pub aim_rotation: f32,
    // Value for forward thrust limit.
    pub speed_limit: f32,
    // How big of a force we have for forward and sideways thrust.
    pub engine_strength: f32,
    pub thrust_braking_strength: f32,
    // Cooldown for main weapon.
    pub main_cooldown: f32,
    pub main_speed: f32,
    // Cargo values
    pub exotic_material: f32,
    pub salvage: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            thrust: 0.0,
            side_braking: 0.0,
            aim_rotation: 0.0,
            speed_limit: 600.0,
            engine_strength: 600.0,
            thrust_braking_strength: 400.0,
            main_cooldown: 0.0,
            main_speed: 1.0 / 2.0,
            exotic_material: 0.0,
            salvage: 0.0,
        }
    }
}

// Define a system to spawn the player.
fn spawn_player(mut commands: Commands, skeletons: Res<Skeletons>) {
    // Spawn a sprite for the player.
    let mut transform = Transform::default();
    transform.scale = Vec3::splat(0.5);
    commands.spawn((
        SpineBundle {
            skeleton: skeletons.player_ship.clone(),
            transform,
            ..Default::default()
        },
        InertiaVolume::new(1.0, 1.0),
        Player::new(),
    ));
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            tonemapping: Tonemapping::Reinhard,
            ..default()
        },
        // BloomSettings::OLD_SCHOOL,
        BloomSettings::default(),
    ));
}

fn player_camera_system(
    mut queries: ParamSet<(
        Query<(&Camera2d, &mut Transform)>,
        Query<(&Player, &Transform)>,
    )>,
) {
    let mut center_transform = Transform::from_xyz(0.0, 0.0, 10.0);
    for (_player, player_transform) in queries.p1().iter() {
        center_transform.translation = player_transform.translation;
    }
    for (_camera, mut transform) in queries.p0().iter_mut() {
        transform.translation = center_transform.translation;
    }
}

fn player_animate_system(mut players: Query<(&Player, &InertiaVolume, &mut Transform)>) {
    for (_player, inertia, mut transform) in players.iter_mut() {
        transform.rotation = Quat::from_rotation_z(inertia.rotation());
    }
}

fn player_movement_system(
    time: Res<Time>,
    mut players: Query<(&mut Player, &mut InertiaVolume)>,
    input: Res<Input<KeyCode>>,
) {
    let dt = time.delta_seconds();
    for (mut player, mut inertia) in players.iter_mut() {
        player.thrust = 0.0;
        player.side_braking = 0.0;
        if input.pressed(KeyCode::W) {
            inertia.apply_thrust_force_limited(player.engine_strength, player.speed_limit, dt);
            player.thrust = player.engine_strength;
            if input.pressed(KeyCode::ShiftLeft) {
                player.side_braking =
                    inertia.apply_thrust_braking(player.thrust_braking_strength, dt);
            } else {
                player.side_braking = 0.0;
            }
        } else if input.pressed(KeyCode::S) {
            inertia.apply_thrust_force_limited(-player.engine_strength, player.speed_limit, dt);
            player.thrust = -player.engine_strength;
            if input.pressed(KeyCode::ShiftLeft) {
                player.side_braking =
                    inertia.apply_thrust_braking(player.thrust_braking_strength, dt);
            } else {
                player.side_braking = 0.0;
            }
        }
        if input.pressed(KeyCode::Q) {
            inertia.apply_offset_thrust_force_limited(
                player.thrust_braking_strength,
                PI / 2.,
                player.speed_limit,
                dt,
            );
            player.side_braking += player.thrust_braking_strength;
            if input.pressed(KeyCode::ShiftLeft)
                && !input.pressed(KeyCode::W)
                && !input.pressed(KeyCode::S)
            {
                player.thrust += inertia.apply_offset_thrust_braking(
                    player.thrust_braking_strength,
                    PI / 2.,
                    dt,
                );
            }
        } else if input.pressed(KeyCode::E) {
            inertia.apply_offset_thrust_force_limited(
                player.thrust_braking_strength,
                -PI / 2.,
                player.speed_limit,
                dt,
            );
            player.side_braking += -player.thrust_braking_strength;
            if input.pressed(KeyCode::ShiftLeft)
                && !input.pressed(KeyCode::W)
                && !input.pressed(KeyCode::S)
            {
                player.thrust +=
                    inertia.apply_offset_thrust_braking(player.engine_strength, -PI / 2., dt);
            }
        }
        if input.pressed(KeyCode::A) {
            inertia.apply_rotation_force(5.0, dt);
        }
        if input.pressed(KeyCode::D) {
            inertia.apply_rotation_force(-5.0, dt);
        }
    }
}

fn get_turret_location(spine: &Spine, turret_name: &'static str) -> Vec2 {
    if let Some(bone) = spine.skeleton.find_bone(turret_name) {
        // "world" is relative to the skeleton, not the whole world.
        bone.world_position().into()
    } else {
        Vec2::ZERO
    }
}

fn get_turret_rotation(spine: &Spine, turret_name: &'static str) -> f32 {
    if let Some(bone) = spine.skeleton.find_bone(turret_name) {
        bone.rotation() * PI / 180.0
    } else {
        0.0
    }
}

fn rotate_turret(spine: &mut Spine, turret_name: &'static str, rotation: f32) {
    if let Some(mut bone) = spine.skeleton.find_bone_mut(turret_name) {
        bone.set_rotation(rotation * 180.0 / PI);
    }
}

fn player_laser_aim_system(
    mut mouse_location: Local<Vec2>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut players: Query<(&mut Player, &Transform, &InertiaVolume, &mut Spine)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let (camera, camera_transform) = camera.single();
    if let Some(cursor_moved_event) = cursor_moved_events.iter().last() {
        *mouse_location = cursor_moved_event.position;
    }
    if let Some(mouse_world_location) = camera
        .viewport_to_world(camera_transform, *mouse_location)
        .map(|v| v.origin.truncate())
    {
        for (mut player, location, my_inertia, mut spine) in players.iter_mut() {
            let turrets = ["left_turret", "right_turret", "forward_turret"];
            for turret_name in turrets.iter() {
                let local_turret_location = get_turret_location(&spine, turret_name);
                let turret_location = location.transform_point(local_turret_location.extend(0.0));
                let turret_direction = mouse_world_location - turret_location.truncate();
                rotate_turret(
                    &mut *spine,
                    turret_name,
                    turret_direction.y.atan2(turret_direction.x) - my_inertia.rotation(),
                );
            }
            let relative_location = (mouse_world_location - location.translation.truncate());
            player.aim_rotation = relative_location.y.atan2(relative_location.x);
            player.aim_rotation -= my_inertia.rotation();
            if player.aim_rotation > PI {
                player.aim_rotation -= 2.0 * PI;
            } else if player.aim_rotation < -PI {
                player.aim_rotation += 2.0 * PI;
            }
        }
    }
}

fn player_laser_fire_system(
    mut commands: Commands,
    time: Res<Time>,
    mut players: Query<(&mut Player, &Transform, &InertiaVolume, &Spine)>,
    lasers: Res<Lasers>,
    input: Res<Input<MouseButton>>,
) {
    for (mut player, location, my_inertia, spine) in players.iter_mut() {
        player.main_cooldown -= time.delta_seconds();
        if input.pressed(MouseButton::Left) {
            if player.main_cooldown <= 0.0 {
                player.main_cooldown = player.main_speed;
                if player.aim_rotation < PI * 5. / 8. && player.aim_rotation > -PI * 5. / 8. {
                    fire_laser_from_turret(
                        "forward_turret",
                        spine,
                        location,
                        my_inertia,
                        &mut commands,
                        &lasers,
                    );
                }
                if player.aim_rotation > -PI / 8. || player.aim_rotation < -PI * 7. / 8. {
                    fire_laser_from_turret(
                        "left_turret",
                        spine,
                        location,
                        my_inertia,
                        &mut commands,
                        &lasers,
                    );
                }
                if player.aim_rotation < PI / 8. || player.aim_rotation > PI * 7. / 8. {
                    fire_laser_from_turret(
                        "right_turret",
                        spine,
                        location,
                        my_inertia,
                        &mut commands,
                        &lasers,
                    );
                }
            }
        }
    }
}

fn fire_laser_from_turret(
    turret_name: &'static str,
    spine: &Spine,
    location: &Transform,
    my_inertia: &InertiaVolume,
    commands: &mut Commands<'_, '_>,
    lasers: &Res<'_, Lasers>,
) {
    // Build a transform for the bullet.
    let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
    // Shoot from the appropriate turret.
    let local_turret_location = get_turret_location(&spine, turret_name);
    transform.translation = location.transform_point(local_turret_location.extend(10.0));
    // Shoot in the appropriate direction.
    let turret_rotation = get_turret_rotation(&spine, turret_name);
    let total_rotation = turret_rotation + my_inertia.rotation();
    let direction = Vec2::new(f32::cos(total_rotation), f32::sin(total_rotation));
    transform.rotation = Quat::from_rotation_z(total_rotation);
    // Give it some speed!
    let mut inertia = InertiaVolume::new(1.0, 1.0);
    inertia.velocity = my_inertia.velocity + direction * 1000.0;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: lasers.player_laser_mesh.clone().into(),
            material: lasers.player_laser_material.clone(),
            transform,
            ..Default::default()
        },
        inertia,
        Bullet::Player,
    ));
}

const JET_BLUENESS: f32 = 24.0;
const JET_BRIGHTNESS: f32 = 10.0;
const JET_ACTIVATION_LIMIT: f32 = 10.0;

fn toggle_jet(mut jet: CTmpMut<Skeleton, Slot>, on: bool) {
    if on {
        jet.color_mut().a = JET_BRIGHTNESS;
        jet.color_mut().b = JET_BLUENESS;
    } else {
        jet.color_mut().a = 0.0;
    }
}

fn player_jet_animation_system(mut players: Query<(&Player, &mut Spine)>) {
    for (player, mut spine) in players.iter_mut() {
        let Spine(SkeletonController { skeleton, .. }) = &mut *spine;
        if let Some(left_brake) = skeleton.find_slot_mut("left_maneuver") {
            toggle_jet(left_brake, player.side_braking < -JET_ACTIVATION_LIMIT);
        }
        if let Some(right_brake) = skeleton.find_slot_mut("right_maneuver") {
            toggle_jet(right_brake, player.side_braking > JET_ACTIVATION_LIMIT);
        }
        if let Some(left_jet) = skeleton.find_slot_mut("left_jet") {
            toggle_jet(left_jet, player.thrust > JET_ACTIVATION_LIMIT);
        }
        if let Some(right_jet) = skeleton.find_slot_mut("right_jet") {
            toggle_jet(right_jet, player.thrust > JET_ACTIVATION_LIMIT);
        }
        if let Some(left_reverse_jet) = skeleton.find_slot_mut("left_reverse_jet") {
            toggle_jet(left_reverse_jet, player.thrust < -JET_ACTIVATION_LIMIT);
        }
        if let Some(right_reverse_jet) = skeleton.find_slot_mut("right_reverse_jet") {
            toggle_jet(right_reverse_jet, player.thrust < -JET_ACTIVATION_LIMIT);
        }
    }
}
