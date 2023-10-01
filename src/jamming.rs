use bevy::sprite::MaterialMesh2dBundle;

use crate::prelude::*;

pub struct JammingPlugin;

impl Plugin for JammingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                deploy_jammer_system.run_if(in_state(GameState::Playing)),
                generate_jamming_pixels.run_if(in_state(GameState::Playing)),
                insert_jammed_around_jammer_system,
                indicate_jamming_on_skeleton,
            ),
        );
    }
}

#[derive(Component)]
pub struct Jammable;

#[derive(Component)]
pub struct Jammed;

#[derive(Component)]
pub struct Jammer {
    pub radius: f32,
    pub progress: f32,
}

pub fn insert_jammed_around_jammer_system(
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(Entity, &Transform), With<Jammable>>,
        Query<(&Transform, &Jammer)>,
    )>,
) {
    let jammables = queries
        .p0()
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect::<Vec<_>>();
    for (jammed_entity, jammed_location) in &jammables {
        if queries.p1().iter().any(|(transform, jammer)| {
            transform.translation.distance(*jammed_location) < jammer.radius
        }) {
            commands.add(InsertSafe {
                entity: *jammed_entity,
                bundle: Jammed,
            });
        } else {
            commands.entity(*jammed_entity).remove::<Jammed>();
        }
    }
}

fn toggle_hyperdrive_enabled(mut slot: CTmpMut<Skeleton, Slot>, enabled: bool, pulse: f32) {
    if enabled {
        slot.color_mut().r = 1.0;
        slot.color_mut().g = 10.0;
        slot.color_mut().b = 1.0;
        slot.color_mut().a = 3.0 + 2.0 * pulse.sin();
    } else {
        slot.color_mut().r = 10.0;
        slot.color_mut().g = 1.0;
        slot.color_mut().b = 1.0;
        slot.color_mut().a = 1.0;
    }
}

pub fn indicate_jamming_on_skeleton(
    mut pulsing: Local<f32>,
    time: Res<Time>,
    mut query: Query<(&mut Spine, Option<&Jammed>), With<Jammable>>,
) {
    *pulsing += time.delta_seconds();
    for (mut spine, jammed) in query.iter_mut() {
        if jammed.is_some() {
            if let Some(mut left) = spine.skeleton.find_slot_mut("hyperdrive_left") {
                toggle_hyperdrive_enabled(left, false, *pulsing);
            }
            if let Some(mut right) = spine.skeleton.find_slot_mut("hyperdrive_right") {
                toggle_hyperdrive_enabled(right, false, *pulsing);
            }
        } else {
            if let Some(mut left) = spine.skeleton.find_slot_mut("hyperdrive_left") {
                toggle_hyperdrive_enabled(left, true, *pulsing);
            }
            if let Some(mut right) = spine.skeleton.find_slot_mut("hyperdrive_right") {
                toggle_hyperdrive_enabled(right, true, *pulsing);
            }
        }
    }
}

const JAMMER_SPAWN_SPEED_PER_R_SQ: f32 = 1000. / (1_000. * 1_000.);

pub fn generate_jamming_pixels(
    time: Res<Time>,
    mut commands: Commands,
    mut jammers: Query<(&mut Jammer, &Transform)>,
    lasers: Res<Lasers>,
) {
    let mut rng = rand::thread_rng();
    let dt = time.delta_seconds();
    for (mut jammer, transform) in jammers.iter_mut() {
        let center = transform.translation;
        let R_sq = jammer.radius * jammer.radius;
        let spawned_pixel_count = R_sq * jammer.progress * JAMMER_SPAWN_SPEED_PER_R_SQ;
        jammer.progress += dt;
        let new_spawned_pixel_count = R_sq * jammer.progress * JAMMER_SPAWN_SPEED_PER_R_SQ;
        let new_pixels = (new_spawned_pixel_count - spawned_pixel_count).floor() as i32;
        for _ in 0..(new_pixels.min(1000)) {
            let r_sq: f32 = rng.gen_range(0.0..1.0);
            let r = jammer.radius * r_sq.sqrt();
            let theta = rng.gen_range(0.0..2.0 * PI);
            let mut transform = Transform::default();
            transform.translation = center + Vec3::new(r * theta.cos(), r * theta.sin(), 0.0);
            commands.spawn((
                MaterialMesh2dBundle {
                    transform,
                    mesh: lasers.jammer_mesh.clone().into(),
                    material: lasers.jammer_material.clone(),
                    ..Default::default()
                },
                SpacePixel::random_jammer(),
            ));
        }
    }
}

pub fn deploy_jammer_system(
    mut cooldown: Local<f32>,
    time: Res<Time>,
    mut commands: Commands,
    mut player: Query<(&mut Player, &Transform, &mut InertiaVolume)>,
    input: Res<Input<KeyCode>>,
    game_assets: Res<GameAssets>,
) {
    *cooldown -= time.delta_seconds();
    if input.just_pressed(KeyCode::G) && *cooldown <= 0.0 {
        let (mut player, player_transform, mut player_inertia) = player.single_mut();
        if player.exotic_material < player.jammer_cost {
            commands.spawn(AudioBundle {
                source: game_assets.fail.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
            return;
        }
        commands.spawn(AudioBundle {
            source: game_assets.deploy_jammer.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
        player.exotic_material -= player.jammer_cost;
        let mut transform = Transform::default();
        transform.translation = player_transform.translation;
        let mut inertia = InertiaVolume::new(1.0, 0.0);
        inertia.velocity = player_inertia.velocity;
        commands.spawn((
            SpriteBundle {
                texture: game_assets.jammer.clone(),
                transform,
                sprite: Sprite {
                    color: Color::rgba(10., 10., 0., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            inertia,
            Regional,
            Jammer {
                radius: 1000.0 * player.jammer_range_multiplier,
                progress: 0.0,
            },
        ));
    }
}
