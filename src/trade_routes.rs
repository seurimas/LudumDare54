use crate::prelude::*;

pub struct TradeRoutesPlugin;

impl Plugin for TradeRoutesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_cargo_ship);
    }
}

fn spawn_cargo_ship(mut commands: Commands, game_assets: Res<GameAssets>) {
    let indicator = commands
        .spawn(ImageBundle {
            image: UiImage::new(game_assets.indicator.clone()),
            style: Style {
                width: Val::Px(16.0),
                height: Val::Px(32.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let indicator_text = commands
        .spawn(TextBundle {
            text: Text::from_section("", TextStyle::default()),
            ..Default::default()
        })
        .id();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: game_assets.cargo_ship.clone(),
            ..Default::default()
        },
        InertiaVolume::new(1.0, 1.0),
        DistantIndicator {
            indicator,
            indicator_text,
        },
    ));
}
