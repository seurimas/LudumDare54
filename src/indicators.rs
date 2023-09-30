use bevy::ui::widget::UiImageSize;

use crate::prelude::*;

const INDICATOR_DISTANCE: f32 = 100.0;
const INDICATOR_TEXT_OFFSET: f32 = 20.0;

pub struct IndicatorsPlugin;

impl Plugin for IndicatorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (display_indicator_system.run_if(in_state(GameState::Playing)),),
        );
    }
}

#[derive(Component, Debug)]
pub struct DistantIndicator {
    pub indicator: Entity,
    pub indicator_text: Entity,
}

fn display_indicator_system(
    windows: Query<&Window>,
    mut queries: ParamSet<(
        Query<&Transform, With<Camera2d>>,
        Query<(&DistantIndicator, &Transform)>,
        Query<(
            &mut Visibility,
            &mut Transform,
            &mut Style,
            Option<&UiImageSize>,
            Option<&mut Text>,
            Option<&bevy::text::TextLayoutInfo>,
        )>,
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
            (
                cargo_ship.indicator,
                cargo_ship.indicator_text,
                Vec3::new(x, y, 0.0),
            )
        })
        .collect::<Vec<_>>();
    for (indicator, indicator_text, direction) in indicator_directions {
        let distance = direction.length();
        let direction = direction.normalize();
        let angle = (-direction.y).atan2(direction.x);
        let visible = if distance > INDICATOR_DISTANCE {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        // Place the indicator image at the edge of the screen
        if let Ok((mut visibility, mut transform, mut style, image_size, _text, _text_layout)) =
            queries.p2().get_mut(indicator)
        {
            *visibility = visible;
            style.left = Val::Px(
                window_size.width() / 2.0 + direction.x * INDICATOR_DISTANCE
                    - image_size.unwrap().size().x / 2.0,
            );
            style.top = Val::Px(
                window_size.height() / 2.0 + -direction.y * INDICATOR_DISTANCE
                    - image_size.unwrap().size().y / 2.0,
            );
            transform.rotation = Quat::from_rotation_z(angle);
        }
        if let Ok((mut visibility, _transform, mut style, _image_size, mut text, text_layout)) =
            queries.p2().get_mut(indicator_text)
        {
            *visibility = visible;
            style.left = Val::Px(
                window_size.width() / 2.0 + direction.x * INDICATOR_DISTANCE
                    - text_layout.unwrap().size.x / 2.0,
            );
            style.top = Val::Px(
                window_size.height() / 2.0
                    + -direction.y * INDICATOR_DISTANCE
                    + INDICATOR_TEXT_OFFSET,
            );
            text.as_mut().unwrap().sections[0].value = format!("{:.0}", distance);
        }
    }
}
