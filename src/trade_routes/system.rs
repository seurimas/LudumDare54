use crate::{
    home::{spawn_home_in_system, HomeInSystem},
    prelude::*,
};

use super::{spawn_cargo_ship, Jammer};

#[derive(Component)]
pub struct SystemLocation {
    pub location: Vec2,
}

impl SystemLocation {
    pub fn new(location: Vec2) -> Self {
        Self { location }
    }
}

#[derive(Component)]
pub struct CurrentSystemRegion;

#[derive(Component)]
pub struct Regional;

#[derive(Component)]
pub struct CargoShipsInSystem(usize);

#[derive(Component)]
pub struct AsteroidsInSystem(usize);

pub fn spawn_starting_system(mut commands: Commands, game_assets: Res<GameAssets>) {
    for _ in 0..10 {
        spawn_asteroids_in_system(
            Vec2::new(
                rand::thread_rng().gen_range((-500.)..500.),
                rand::thread_rng().gen_range((-500.)..500.),
            ),
            &game_assets,
            &mut commands,
            rand::thread_rng().gen_range(5..8),
        );
    }
    for _ in 0..10 {
        spawn_cargo_ships_in_system(
            Vec2::new(
                rand::thread_rng().gen_range((-500.)..500.),
                rand::thread_rng().gen_range((-500.)..500.),
            ),
            &game_assets,
            &mut commands,
            1,
        );
    }
    spawn_home_in_system(
        Vec2::new(
            rand::thread_rng().gen_range((-500.)..500.),
            rand::thread_rng().gen_range((-500.)..500.),
        ),
        &game_assets,
        &mut commands,
    );
}

pub fn spawn_asteroid_field(mut commands: Commands, game_assets: Res<GameAssets>, count: usize) {
    for _ in 0..count {
        spawn_exotic(
            rand::thread_rng().gen_range((-ARENA_SIZE)..ARENA_SIZE),
            rand::thread_rng().gen_range((-ARENA_SIZE)..ARENA_SIZE),
            &mut commands,
            game_assets.exotic.clone(),
            rand::thread_rng().gen_range(5.0..15.0),
        )
    }
}

fn spawn_asteroids_in_system(
    location: Vec2,
    game_assets: &Res<GameAssets>,
    mut commands: &mut Commands,
    size: usize,
) {
    let (indicator, indicator_text) = create_indicator_with_text(commands, game_assets, false);
    commands.spawn((
        TransformBundle::default(),
        SystemLocation { location },
        AsteroidsInSystem(size),
        DistantIndicator::new_system(indicator, indicator_text, Vec2::ZERO),
    ));
}

fn spawn_cargo_ships_in_system(
    location: Vec2,
    game_assets: &Res<GameAssets>,
    mut commands: &mut Commands,
    size: usize,
) {
    let (indicator, indicator_text) = create_indicator_with_text(commands, game_assets, false);
    commands.spawn((
        TransformBundle::default(),
        SystemLocation { location },
        CargoShipsInSystem(size),
        DistantIndicator::new_system(indicator, indicator_text, Vec2::ZERO),
    ));
}

pub fn pick_hyperdrive_target(
    mut player: Query<(&mut Player, &InertiaVolume)>,
    mut indicators: Query<(Entity, &DistantIndicator), Without<CurrentSystemRegion>>,
) {
    let mut min_distance = f32::MAX;
    let mut hyperdrive_target = None;
    let player_facing = player.single().1.rotation();
    let player_facing = Vec2::new(player_facing.cos(), player_facing.sin());
    for (system_entity, indicator) in indicators.iter_mut() {
        match indicator {
            DistantIndicator::System {
                visible, direction, ..
            } => {
                let distance = direction.normalize().distance(player_facing);
                if *visible && distance < min_distance {
                    min_distance = distance;
                    hyperdrive_target = Some(system_entity);
                }
            }
            _ => {}
        }
    }
    player.single_mut().0.hyperdrive_target = hyperdrive_target;
}

const MAX_HYPERDRIVE_TARGETS: usize = 5;

pub fn update_system_indicators(
    player: Query<(&Player, &SystemLocation)>,
    mut indicators: Query<(
        Entity,
        &SystemLocation,
        &mut DistantIndicator,
        Option<&AsteroidsInSystem>,
        Option<&CargoShipsInSystem>,
        Option<&HomeInSystem>,
        Option<&CurrentSystemRegion>,
    )>,
    mut indicator_texts: Query<&mut Text>,
) {
    let hyperdrive_target = player.single().0.hyperdrive_target;
    let player_location = player.single().1.location;
    let mut sorted_indicators = indicators
        .iter_mut()
        .map(|(entity, system_location, ..)| {
            (entity, system_location.location.distance(player_location))
        })
        .collect::<Vec<_>>();
    sorted_indicators.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    let mut idx = 0;
    for (entity, distance) in sorted_indicators.iter() {
        if let Ok((_, system_location, mut indicator, m_asteroid, m_ship, m_home, m_current)) =
            indicators.get_mut(*entity)
        {
            match &mut *indicator {
                DistantIndicator::System {
                    indicator_text,
                    visible,
                    direction,
                    ..
                } => {
                    *direction = player_location - system_location.location;
                    if let Ok(mut indicator_text) = indicator_texts.get_mut(*indicator_text) {
                        indicator_text.sections[0].value = format!(
                            "{:.2}AU ({})",
                            distance,
                            if m_asteroid.is_some() {
                                "Asteroids"
                            } else if m_ship.is_some() {
                                "Cargo Ships"
                            } else if m_home.is_some() {
                                "Hideout"
                            } else {
                                "Unknown"
                            }
                        );
                        if hyperdrive_target == Some(*entity) {
                            indicator_text.sections[0].style.color = Color::YELLOW;
                        } else {
                            indicator_text.sections[0].style.color = Color::WHITE;
                        }
                    }
                    if m_current.is_some() {
                        *visible = false;
                    } else if m_home.is_some() {
                        *visible = true;
                    } else {
                        *visible = idx < MAX_HYPERDRIVE_TARGETS;
                        idx += 1;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn engage_hyperdrive_system(
    mut cooldown: Local<f32>,
    time: Res<Time>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut player: Query<(&Player, &mut InertiaVolume, Option<&Jammed>)>,
    input: Res<Input<KeyCode>>,
    game_assets: Res<GameAssets>,
) {
    *cooldown -= time.delta_seconds();
    if input.just_pressed(KeyCode::Space) && *cooldown <= 0.0 {
        let (player, mut player_inertia, m_jammed) = player.single_mut();
        if player_inertia.forward_speed() < HYPERDRIVE_SPEED {
            // TODO: Indicate failure.
            return;
        }
        if m_jammed.is_some() {
            commands.spawn(AudioBundle {
                source: game_assets.player_jammed.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
            return;
        }
        if let Some(_hyperdrive_target) = player.hyperdrive_target {
            commands.spawn(AudioBundle {
                source: game_assets.player_hyperdrive.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
            player_inertia.set_forward_speed(HYPERDRIVE_SPEED * 2.);
            next_state.set(GameState::Hyperdrive);
            *cooldown = 2.0;
        }
    }
}

pub fn initialize_local_region(
    mut timeout: Local<f32>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player: Query<(Entity, &Player, &mut InertiaVolume, &mut Transform)>,
    mut system_locations: Query<&mut SystemLocation>,
    regions: Query<(
        Option<&AsteroidsInSystem>,
        Option<&CargoShipsInSystem>,
        Option<&HomeInSystem>,
    )>,
    regional_entities: Query<(Entity, &Regional)>,
    time: Res<Time>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    skeletons: Res<Skeletons>,
) {
    if *timeout == 0. {
        *timeout = 5.;
    }
    if *timeout > 0. && *timeout < time.delta_seconds() {
        // Initialize the area!
        let (player_entity, player, mut inertia, mut player_transform) = player.single_mut();
        let new_region = player.hyperdrive_target.unwrap();
        // Move the player to the new region.
        let player_location = system_locations.get(player_entity).unwrap().location;
        let target_location = system_locations.get(new_region).unwrap().location;
        system_locations.get_mut(player_entity).unwrap().location = target_location;
        // Set their local position and velocity based on their travel direction.
        let incoming_direction = (target_location - player_location).normalize();
        player_transform.translation = (incoming_direction * ARENA_SIZE).extend(0.0);
        inertia.velocity = -incoming_direction * HYPERDRIVE_SPEED;
        // Despawn all regional entities.
        for (entity, _) in regional_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }

        match regions.get(new_region).unwrap() {
            (Some(asteroids), _, _) => {
                spawn_asteroid_field(commands, game_assets, asteroids.0);
                next_state.set(GameState::Playing);
            }
            (_, Some(cargo_ships), _) => {
                spawn_cargo_ship(commands, game_assets, skeletons);
                next_state.set(GameState::Playing);
            }
            (_, _, Some(_)) => {
                next_state.set(GameState::Home);
            }
            _ => {
                next_state.set(GameState::Playing);
            }
        }

        // Switch game states, we're ready to go.
        *timeout = 5.;
    }
    *timeout -= time.delta_seconds();
}
