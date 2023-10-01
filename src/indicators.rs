use bevy::ui::widget::UiImageSize;

use crate::{game_state, prelude::*};

const INDICATOR_DISTANCE: f32 = 300.0;
const INDICATOR_TEXT_OFFSET: f32 = 20.0;

pub struct IndicatorsPlugin;

impl Plugin for IndicatorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (display_indicator_system.run_if(not(in_state(GameState::Loading))),),
        );
    }
}

#[derive(Component, Debug)]
pub enum DistantIndicator {
    Local {
        indicator: Entity,
        indicator_text: Entity,
    },
    System {
        indicator: Entity,
        indicator_text: Entity,
        direction: Vec2,
        visible: bool,
    },
}

impl DistantIndicator {
    pub fn new_local(indicator: Entity, indicator_text: Entity) -> Self {
        Self::Local {
            indicator,
            indicator_text,
        }
    }

    pub fn new_system(indicator: Entity, indicator_text: Entity, direction: Vec2) -> Self {
        Self::System {
            indicator,
            indicator_text,
            direction,
            visible: false,
        }
    }

    pub fn get_indicator(&self) -> Entity {
        match self {
            Self::Local {
                indicator,
                indicator_text: _,
            } => *indicator,
            Self::System {
                indicator,
                indicator_text: _,
                direction: _,
                visible: _,
            } => *indicator,
        }
    }

    pub fn get_indicator_text(&self) -> Entity {
        match self {
            Self::Local {
                indicator: _,
                indicator_text,
            } => *indicator_text,
            Self::System {
                indicator: _,
                indicator_text,
                direction: _,
                visible: _,
            } => *indicator_text,
        }
    }
}

pub fn create_indicator_with_text(
    commands: &mut Commands,
    game_assets: &GameAssets,
    regional: bool,
) -> (Entity, Entity) {
    let indicator = commands
        .spawn(ImageBundle {
            image: UiImage::new(game_assets.indicator.clone()),
            style: Style {
                position_type: PositionType::Absolute,
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
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    if regional {
        commands.entity(indicator).insert(Regional);
        commands.entity(indicator_text).insert(Regional);
    }
    (indicator, indicator_text)
}

fn display_indicator_system(
    windows: Query<&Window>,
    game_state: Res<State<GameState>>,
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
        .map(|(indicator, transform)| match indicator {
            DistantIndicator::Local {
                indicator,
                indicator_text,
            } => {
                let translation = transform.translation;
                let x = translation.x - camera_translation.x;
                let y = translation.y - camera_translation.y;
                let direction = Vec3::new(x, y, 0.0);
                (
                    direction.length() > INDICATOR_DISTANCE,
                    *indicator,
                    *indicator_text,
                    direction,
                )
            }
            DistantIndicator::System {
                visible,
                indicator,
                indicator_text,
                direction,
            } => (*visible, *indicator, *indicator_text, direction.extend(0.)),
        })
        .collect::<Vec<_>>();
    for (visible, indicator, indicator_text, direction) in indicator_directions {
        let direction = direction.normalize();
        let angle = (-direction.y).atan2(direction.x);
        let visible = if *game_state == GameState::Playing && visible {
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
        }
    }
}
