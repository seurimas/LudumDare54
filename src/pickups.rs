use crate::prelude::*;

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_debug_drops);
    }
}

fn spawn_debug_drops(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            texture: game_assets.drop.clone(),
            ..Default::default()
        },
        InertiaVolume::new(1.0, 1.0),
    ));
}
