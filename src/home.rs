use crate::prelude::*;

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct HomeInSystem;

pub fn spawn_home_in_system(
    location: Vec2,
    game_assets: &Res<GameAssets>,
    mut commands: &mut Commands,
) {
    let (indicator, indicator_text) = create_indicator_with_text(commands, game_assets, false);
    commands.spawn((
        TransformBundle::default(),
        SystemLocation { location },
        HomeInSystem,
        DistantIndicator::new_system(indicator, indicator_text, Vec2::ZERO),
    ));
}
