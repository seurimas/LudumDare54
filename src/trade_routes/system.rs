use crate::prelude::*;

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
            rand::thread_rng().gen_range(3..6),
        );
    }
}

fn spawn_asteroids_in_system(
    location: Vec2,
    game_assets: &Res<GameAssets>,
    mut commands: &mut Commands,
    size: usize,
) {
    let (indicator, indicator_text) = create_indicator_with_text(commands, game_assets);
    commands.spawn((
        TransformBundle::default(),
        SystemLocation { location },
        AsteroidsInSystem(size),
        DistantIndicator::new_system(indicator, indicator_text, Vec2::ZERO),
    ));
}

pub fn pick_hyperdrive_target(
    mut player: Query<(&mut Player, &InertiaVolume)>,
    mut indicators: Query<(Entity, &DistantIndicator)>,
) {
    let mut min_distance = f32::MAX;
    let mut hyperdrive_target = None;
    let player_facing = player.single().1.rotation();
    let player_facing = Vec2::new(player_facing.cos(), player_facing.sin());
    for (system_entity, indicator) in indicators.iter_mut() {
        match indicator {
            DistantIndicator::System { direction, .. } => {
                let distance = direction.normalize().distance(player_facing);
                if distance < min_distance {
                    min_distance = distance;
                    hyperdrive_target = Some(system_entity);
                }
            }
            _ => {}
        }
    }
    player.single_mut().0.hyperdrive_target = hyperdrive_target;
}

pub fn update_system_indicators(
    player: Query<(&Player, &SystemLocation)>,
    mut indicators: Query<(
        Entity,
        &SystemLocation,
        &mut DistantIndicator,
        Option<&AsteroidsInSystem>,
        Option<&CargoShipsInSystem>,
    )>,
    mut indicator_texts: Query<&mut Text>,
) {
    let hyperdrive_target = player.single().0.hyperdrive_target;
    let player_location = player.single().1.location;
    for (system_entity, system_location, mut indicator, m_asteroid, m_ship) in indicators.iter_mut()
    {
        match &mut *indicator {
            DistantIndicator::System {
                direction,
                indicator_text,
                ..
            } => {
                let direction_to_system = system_location.location - player_location;
                *direction = direction_to_system;
                let distance = direction_to_system.length();
                if let Ok(mut indicator_text) = indicator_texts.get_mut(*indicator_text) {
                    indicator_text.sections[0].value = format!(
                        "{:.2}AU ({})",
                        distance,
                        if m_asteroid.is_some() {
                            "Asteroids"
                        } else if m_ship.is_some() {
                            "Cargo Ships"
                        } else {
                            "Unknown"
                        }
                    );
                    if hyperdrive_target == Some(system_entity) {
                        indicator_text.sections[0].style.color = Color::YELLOW;
                    } else {
                        indicator_text.sections[0].style.color = Color::WHITE;
                    }
                }
            }
            _ => {}
        }
    }
}
