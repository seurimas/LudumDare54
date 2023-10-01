use std::f32::consts::PI;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    transform::TransformSystem,
};

use crate::{assets::Skeletons, prelude::*};

// Define a plugin for the player.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add the player to the world.
        app.add_systems(
            OnTransition {
                from: GameState::Loading,
                to: GameState::Playing,
            },
            spawn_player,
        )
        .add_systems(
            PostUpdate,
            player_camera_system
                .before(TransformSystem::TransformPropagate)
                .run_if(not(in_state(GameState::Hyperdrive))),
        )
        .add_systems(
            Update,
            (
                player_movement_system.run_if(in_state(GameState::Playing)),
                player_laser_aim_system.run_if(in_state(GameState::Playing)),
                player_laser_fire_system.run_if(in_state(GameState::Playing)),
                player_shield_recharge_system,
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
    // Hyperdrive
    pub hyperdrive_target: Option<Entity>,
    // Health.
    pub max_shields: f32,
    pub shields: f32,
    pub max_hull: f32,
    pub hull: f32,
    pub shield_recharge_delay: f32,
    pub shield_recharge_timer: f32,
    pub shield_recharge_rate: f32,
    // Cargo.
    pub max_cargo: f32,
    pub salvage_mass: f32,
    pub salvage_value: f32,
    pub exotic_material: f32,
    pub upgrade_material: Option<Upgrade>,
    // Upgrades
    pub jammer_range_multiplier: f32,
    pub jammer_cost: f32,
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
            hyperdrive_target: None,
            max_shields: 100.0,
            shields: 100.0,
            max_hull: 100.0,
            hull: 100.0,
            shield_recharge_delay: 5.0,
            shield_recharge_timer: 0.0,
            shield_recharge_rate: 10.0,
            max_cargo: 100.0,
            salvage_mass: 0.0,
            salvage_value: 0.0,
            exotic_material: 0.0,
            upgrade_material: None,
            jammer_range_multiplier: 1.0,
            jammer_cost: 20.0,
        }
    }

    pub fn apply_upgrade(&mut self, upgrade: Upgrade) {
        match upgrade {
            Upgrade::EngineUpgrade => {
                self.speed_limit *= 1.1;
                self.engine_strength *= 1.1;
                self.thrust_braking_strength *= 1.1;
            }
            Upgrade::ShieldStrength => {
                self.max_shields *= 1.1;
            }
            Upgrade::HullStrength => {
                self.max_hull *= 1.1;
            }
            Upgrade::ShieldRecharge => {
                self.shield_recharge_delay *= 0.9;
                self.shield_recharge_rate *= 1.1;
            }
            Upgrade::FireSpeed => {
                self.main_speed *= 0.9;
            }
            Upgrade::JammerRange => {
                self.jammer_range_multiplier *= 1.1;
            }
            Upgrade::JammerEfficiency => {
                if self.jammer_cost <= 1.0 {
                    self.jammer_cost *= 0.9;
                } else {
                    self.jammer_cost -= 1.0;
                }
            }
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.shield_recharge_timer = self.shield_recharge_delay;
        if self.shields > 0.0 {
            self.shields -= amount;
        } else {
            self.hull -= amount;
        }
    }

    pub fn cargo_space_left(&self) -> f32 {
        (self.max_cargo - self.salvage_mass - self.exotic_material).max(0.)
    }

    pub fn get_repair_cost(&self) -> f32 {
        self.max_hull - self.hull
    }

    pub fn repair(&mut self, amount: f32) {
        self.shields = self.max_shields;
        self.hull += amount;
        if self.hull > self.max_hull {
            self.hull = self.max_hull;
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
        InertiaVolume::new(1.0, 64.0),
        Player::new(),
        SystemLocation::new(Vec2::new(0.0, 0.0)),
        Jammable,
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
            if input.pressed(KeyCode::ShiftLeft) {
                inertia.apply_thrust_force(player.engine_strength, dt);
                player.thrust = player.engine_strength;
                player.side_braking =
                    inertia.apply_thrust_braking(player.thrust_braking_strength, dt);
            } else {
                inertia.apply_thrust_force_limited(player.engine_strength, player.speed_limit, dt);
                player.thrust = player.engine_strength;
                player.side_braking = 0.0;
            }
        } else if input.pressed(KeyCode::S) {
            if input.pressed(KeyCode::ShiftLeft) {
                inertia.apply_thrust_force(-player.engine_strength, dt);
                player.thrust = player.engine_strength;
                player.side_braking =
                    inertia.apply_thrust_braking(player.thrust_braking_strength, dt);
            } else {
                inertia.apply_thrust_force_limited(-player.engine_strength, player.speed_limit, dt);
                player.thrust = -player.engine_strength;
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
                rotate_towards_world_location(
                    &mut spine,
                    *turret_name,
                    location,
                    mouse_world_location,
                    my_inertia,
                );
            }
            let relative_location = mouse_world_location - location.translation.truncate();
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
                        lasers.player_laser_mesh.clone().into(),
                        lasers.player_laser_material.clone(),
                        Bullet::Player,
                    );
                }
                if player.aim_rotation > -PI / 8. || player.aim_rotation < -PI * 7. / 8. {
                    fire_laser_from_turret(
                        "left_turret",
                        spine,
                        location,
                        my_inertia,
                        &mut commands,
                        lasers.player_laser_mesh.clone().into(),
                        lasers.player_laser_material.clone(),
                        Bullet::Player,
                    );
                }
                if player.aim_rotation < PI / 8. || player.aim_rotation > PI * 7. / 8. {
                    fire_laser_from_turret(
                        "right_turret",
                        spine,
                        location,
                        my_inertia,
                        &mut commands,
                        lasers.player_laser_mesh.clone().into(),
                        lasers.player_laser_material.clone(),
                        Bullet::Player,
                    );
                }
            }
        }
    }
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

fn player_shield_recharge_system(time: Res<Time>, mut players: Query<(&mut Player)>) {
    for mut player in players.iter_mut() {
        if player.shield_recharge_timer > 0.0 {
            player.shield_recharge_timer -= time.delta_seconds();
        } else {
            player.shields += player.shield_recharge_rate * time.delta_seconds();
            if player.shields > player.max_shields {
                player.shields = player.max_shields;
            }
        }
    }
}
