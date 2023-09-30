use bevy::ui::widget::UiImageSize;

use crate::prelude::*;

const INDICATOR_DISTANCE: f32 = 100.0;

pub struct CargoShipPlugin;

impl Plugin for CargoShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_cargo_ship)
            .add_systems(
                Update,
                (cargo_ship_indicator_system.run_if(in_state(GameState::Playing)),),
            );
    }
}

#[derive(Component, Debug)]
pub struct CargoShip {
    pub indicator: Entity,
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
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: game_assets.cargo_ship.clone(),
            ..Default::default()
        },
        InertiaVolume::new(Vec2::new(0.0, 0.0), 0.0, 1.0, 1.0),
        CargoShip { indicator },
    ));
}

fn cargo_ship_indicator_system(
    windows: Query<&Window>,
    mut queries: ParamSet<(
        Query<&Transform, With<Camera2d>>,
        Query<(&CargoShip, &Transform)>,
        Query<(&mut Visibility, &mut Transform, &mut Style)>,
    )>,
) {
    let camera_translation = queries.p0().single().translation;
    let window_size = &windows.single().resolution;
    let indicator_directions = queries
        .p1()
        .iter()
        .map(|(cargo_ship, transform)| {
            let translation = transform.translation;
            let x = translation.x - camera_translation.x;
            let y = translation.y - camera_translation.y;
            (cargo_ship.indicator, Vec3::new(x, y, 0.0))
        })
        .collect::<Vec<_>>();
    for (indicator, direction) in indicator_directions {
        if let Ok((mut visibility, mut transform, mut style)) = queries.p2().get_mut(indicator) {
            let distance = direction.length();
            let direction = direction.normalize();
            let angle = (-direction.y).atan2(direction.x);
            *visibility = if distance > INDICATOR_DISTANCE {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            style.left = Val::Px(window_size.width() / 2.0 + direction.x * INDICATOR_DISTANCE);
            style.top = Val::Px(window_size.height() / 2.0 + -direction.y * INDICATOR_DISTANCE);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
