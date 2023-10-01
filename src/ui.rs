use bevy::text::DEFAULT_FONT_HANDLE;

use crate::prelude::*;

const CARGO_CELL_COUNT: usize = 100;
const CARGO_CELL_COLUMNS: usize = 20;
const CARGO_CELL_ROWS: usize = CARGO_CELL_COUNT / CARGO_CELL_COLUMNS;
const CARGO_CELL_SIZE: f32 = 16.;

pub struct GameUiPlugin;

#[derive(Resource)]
pub struct UiState {
    shield_display: Entity,
    hull_display: Entity,
    cargo_display: Entity,
    cargo_text: Entity,
    cargo_cells: Vec<Entity>,
    upgrade_text: Entity,
    pub home_text: Entity,
}

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), setup_ui)
            .add_systems(
                PostUpdate,
                update_ui.run_if(not(in_state(GameState::Loading))),
            );
    }
}

fn setup_ui(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Health display.
    let shield_display = commands
        .spawn((TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                width: Val::Px(230.),
                height: Val::Px(30.),
                ..Default::default()
            },
            background_color: Color::rgba(0., 0., 1., 1.0).into(),
            text: Text::from_section(
                "Shields: 100%",
                TextStyle {
                    font: DEFAULT_FONT_HANDLE.typed(),
                    font_size: 30.,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..Default::default()
        },))
        .id();
    let hull_display = commands
        .spawn((TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(30.),
                width: Val::Px(230.),
                height: Val::Px(20.),
                ..Default::default()
            },
            background_color: Color::rgba(1., 0., 0., 1.0).into(),
            text: Text::from_section(
                "Hull: 100%",
                TextStyle {
                    font: DEFAULT_FONT_HANDLE.typed(),
                    font_size: 20.,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..Default::default()
        },))
        .id();
    // Display cargo!
    let mut cargo_cells = Vec::new();
    let mut cargo_text = None;
    let cargo_display = commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Grid,
                grid_template_rows: RepeatedGridTrack::flex(CARGO_CELL_ROWS as u16, 1.),
                grid_template_columns: RepeatedGridTrack::flex(CARGO_CELL_COLUMNS as u16, 1.),
                left: Val::Px(0.),
                top: Val::Px(70.),
                height: Val::Px(CARGO_CELL_SIZE * CARGO_CELL_ROWS as f32),
                width: Val::Px(CARGO_CELL_SIZE * CARGO_CELL_COLUMNS as f32),
                ..Default::default()
            },
            background_color: Color::rgba(0.4, 0.4, 0.4, 1.0).into(),
            ..Default::default()
        },))
        .with_children(|builder| {
            cargo_text = Some(
                builder
                    .spawn(TextBundle {
                        z_index: ZIndex::Local(100),
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.),
                            top: Val::Px(-20.),
                            height: Val::Px(20.),
                            ..Default::default()
                        },
                        text: Text::from_sections(vec![
                            TextSection {
                                value: "Cargo: 0/0".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "Value: 0/0".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                        ])
                        .with_alignment(TextAlignment::Center),
                        ..Default::default()
                    })
                    .id(),
            );
            for _ in 0..CARGO_CELL_COUNT {
                cargo_cells.push(
                    builder
                        .spawn(NodeBundle {
                            border_color: Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
                            style: Style {
                                display: Display::Grid,
                                width: Val::Px(CARGO_CELL_SIZE),
                                height: Val::Px(CARGO_CELL_SIZE),
                                border: UiRect::all(Val::Px(1.0)),
                                ..Default::default()
                            },
                            background_color: Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
                            ..Default::default()
                        })
                        .id(),
                );
            }
        })
        .id();
    let upgrade_text = commands
        .spawn((TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(170.),
                width: Val::Px(230.),
                height: Val::Px(20.),
                ..Default::default()
            },
            text: Text::from_section(
                "Upgrade found:",
                TextStyle {
                    font: DEFAULT_FONT_HANDLE.typed(),
                    font_size: 20.,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..Default::default()
        },))
        .id();
    // End cargo.
    // Home text.
    let mut home_text = None;
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            home_text = Some(
                builder
                    .spawn(TextBundle {
                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::from_sections(vec![
                            TextSection {
                                value: "Earnings: ".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "XXX".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "\nRepair costs: ".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "XXX".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "\nUpgrade discovered: ".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "XXX".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "\n10 days left".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "\nPress <Space> to continue".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
                        ])
                        .with_alignment(TextAlignment::Center),
                        ..Default::default()
                    })
                    .id(),
            )
        });

    // Save everything to the resource.
    commands.insert_resource(UiState {
        shield_display,
        hull_display,
        cargo_display,
        cargo_text: cargo_text.unwrap(),
        cargo_cells,
        upgrade_text,
        home_text: home_text.unwrap(),
    });
}

fn update_ui(
    ui_state: Res<UiState>,
    mut bg_color: Query<&mut BackgroundColor>,
    mut text: Query<&mut Text>,
    player: Query<&Player>,
) {
    let player = player.single();
    // Display health!
    if let Ok(mut shield_text) = text.get_mut(ui_state.shield_display) {
        let shield_percent = player.shields / player.max_shields * 100.;
        shield_text.sections[0].value = format!("Shields: {}", player.shields.floor() as i32);
        if let Ok(mut bg_color) = bg_color.get_mut(ui_state.shield_display) {
            bg_color.0 = Color::rgba(0.0, 1.0 - shield_percent / 100., shield_percent / 100., 1.0);
        }
    }
    if let Ok(mut hull_text) = text.get_mut(ui_state.hull_display) {
        let hull_percent = player.hull / player.max_hull * 100.;
        hull_text.sections[0].value = format!("Hull: {}", player.hull.floor() as i32);
        if let Ok(mut bg_color) = bg_color.get_mut(ui_state.hull_display) {
            bg_color.0 = Color::rgba(hull_percent / 100., 1.0 - hull_percent / 100., 0.0, 1.0);
        }
    }
    // End health.

    // Display cargo!
    const EXOTIC_COLOR: Color = Color::rgba(1.0, 1.0, 0.0, 1.0);
    const SALVAGE_COLOR: Color = Color::rgba(0.5, 0.5, 0.5, 1.0);
    let mut exotic_drawn = 0;
    let mut salvage_drawn = 0;
    let exotics = player.exotic_material.floor() as i32;
    let salvage = player.salvage_mass.ceil() as i32;
    for cell in ui_state.cargo_cells.iter() {
        if let Ok(mut cell_bg) = bg_color.get_mut(*cell) {
            if exotic_drawn < exotics {
                exotic_drawn += 1;
                cell_bg.0 = EXOTIC_COLOR;
            } else if salvage_drawn < salvage {
                salvage_drawn += 1;
                cell_bg.0 = SALVAGE_COLOR;
            } else {
                cell_bg.0 = Color::rgba(0.0, 0.0, 0.0, 1.0);
            }
        }
    }
    if let Ok(mut cargo_text) = text.get_mut(ui_state.cargo_text) {
        cargo_text.sections[0].value = format!("Cargo: {}/{}", exotics + salvage, CARGO_CELL_COUNT);
        cargo_text.sections[1].value = format!(" Value: {}", player.salvage_value.floor() as i32);
    }
    if let Ok(mut upgrade_text) = text.get_mut(ui_state.upgrade_text) {
        if let Some(upgrade) = &player.upgrade_material {
            upgrade_text.sections[0].value =
                format!("Upgrade found: {}", upgrade.get_upgrade_material_name());
        } else {
            upgrade_text.sections[0].value = "".to_string();
        }
    }
}
