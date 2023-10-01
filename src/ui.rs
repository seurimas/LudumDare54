use bevy::text::DEFAULT_FONT_HANDLE;

use crate::{home::Career, prelude::*};

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
    pub central_text: Entity,
    pub game_over_text: Entity,
    pub retire_text: Entity,
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

fn spawn_centered_text(
    commands: &mut Commands,
    sections: Vec<String>,
    font_size: f32,
    color: Color,
) -> Entity {
    let mut text_entity = None;
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
            text_entity = Some(
                builder
                    .spawn(TextBundle {
                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::from_sections(sections.iter().map(|text| TextSection {
                            value: text.to_string(),
                            style: TextStyle {
                                font: DEFAULT_FONT_HANDLE.typed(),
                                font_size,
                                color,
                            },
                        }))
                        .with_alignment(TextAlignment::Center),
                        ..Default::default()
                    })
                    .id(),
            )
        });
    text_entity.unwrap()
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
                top: Val::Px(90.),
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
                            top: Val::Px(-40.),
                            height: Val::Px(40.),
                            width: Val::Px(400.),
                            ..Default::default()
                        },
                        text: Text::from_sections(vec![
                            TextSection {
                                value: "Press [G] to deploy jammer.\n".to_string(),
                                style: TextStyle {
                                    font: DEFAULT_FONT_HANDLE.typed(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            },
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
                        .with_alignment(TextAlignment::Left),
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
                width: Val::Px(400.),
                height: Val::Px(200.),
                ..Default::default()
            },
            text: Text::from_section(
                "Upgrade found:",
                TextStyle {
                    font: DEFAULT_FONT_HANDLE.typed(),
                    font_size: 16.,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Left),
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
    // End home text.
    let game_over_text = spawn_centered_text(
        &mut commands,
        vec!["GAME OVER".to_string(), "".to_string(), "".to_string()],
        40.,
        Color::RED,
    );
    let retire_text = spawn_centered_text(
        &mut commands,
        vec![
            "Happy retirement!\n\n".to_string(),
            "".to_string(),
            "".to_string(),
        ],
        40.,
        Color::YELLOW_GREEN,
    );
    // Central text.
    let mut central_text = None;
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            central_text = Some(
                builder
                    .spawn(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::from_sections(vec![TextSection {
                            value: "Some text".to_string(),
                            style: TextStyle {
                                font: DEFAULT_FONT_HANDLE.typed(),
                                font_size: 40.,
                                color: Color::WHITE,
                            },
                        }])
                        .with_alignment(TextAlignment::Center),
                        ..Default::default()
                    })
                    .id(),
            )
        });
    // End home text.

    // Save everything to the resource.
    commands.insert_resource(UiState {
        shield_display,
        hull_display,
        cargo_display,
        cargo_text: cargo_text.unwrap(),
        cargo_cells,
        upgrade_text,
        home_text: home_text.unwrap(),
        central_text: central_text.unwrap(),
        game_over_text,
        retire_text,
    });
}

fn update_ui(
    game_state: Res<State<GameState>>,
    ui_state: Res<UiState>,
    career: Res<Career>,
    mut bg_color: Query<&mut BackgroundColor>,
    mut visibility: Query<&mut Visibility>,
    mut text: Query<&mut Text>,
    player: Query<(&Player, &InertiaVolume, Option<&Jammed>)>,
) {
    let (player, player_inertia, m_player_jammed) = player.single();
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
    const UPGRADE_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    let mut exotic_drawn = 0;
    let mut upgrades_drawn = 0;
    let mut salvage_drawn = 0;
    let exotics = player.exotic_material.floor() as i32;
    let upgrades = player.upgrade_mass.ceil() as i32;
    let salvage = player.salvage_mass.ceil() as i32;
    for cell in ui_state.cargo_cells.iter() {
        if let Ok(mut cell_bg) = bg_color.get_mut(*cell) {
            if exotic_drawn < exotics {
                exotic_drawn += 1;
                cell_bg.0 = EXOTIC_COLOR;
            } else if upgrades_drawn < upgrades {
                upgrades_drawn += 1;
                cell_bg.0 = UPGRADE_COLOR;
            } else if salvage_drawn < salvage {
                salvage_drawn += 1;
                cell_bg.0 = SALVAGE_COLOR;
            } else {
                cell_bg.0 = Color::rgba(0.0, 0.0, 0.0, 1.0);
            }
        }
    }
    if let Ok(mut cargo_text) = text.get_mut(ui_state.cargo_text) {
        if player.exotic_material >= player.jammer_cost {
            cargo_text.sections[0].value = "Press [G] to deploy jammer.\n".to_string();
        } else {
            cargo_text.sections[0].value = "Gather XM to create jammers.\n".to_string();
        }
        cargo_text.sections[1].value = format!(
            "Cargo: {}/{}",
            exotics + salvage + upgrades,
            CARGO_CELL_COUNT
        );
        cargo_text.sections[2].value = format!(" Value: ${}", player.salvage_value.floor() as i32);
    }
    if let Ok(mut upgrade_text) = text.get_mut(ui_state.upgrade_text) {
        upgrade_text.sections[0].value = "".to_string();
        for upgrade in player.upgrade_materials.iter() {
            upgrade_text.sections[0].value = format!(
                "{}\nXM item found: {}",
                upgrade_text.sections[0].value,
                upgrade.get_upgrade_material_name()
            );
        }
    }
    if let Ok(mut central_text) = text.get_mut(ui_state.central_text) {
        if *game_state == GameState::Home
            || *game_state == GameState::Retire
            || *game_state == GameState::GameOver
        {
            central_text.sections[0].value = "".to_string();
        } else if !career.intro_complete {
            // Let the intro system handle it.
        } else if m_player_jammed.is_some() {
            central_text.sections[0].value = format!("Hyperdrive JAMMED! Leave jamming area!");
        } else if player_inertia.forward_speed() < HYPERDRIVE_SPEED {
            central_text.sections[0].value = format!("Increase speed to engage hyperdrive!");
        } else {
            central_text.sections[0].value = format!("Press [SPACE] to engage hyperdrive!");
        }
    }
    if let Ok(mut visible) = visibility.get_mut(ui_state.game_over_text) {
        if *game_state == GameState::GameOver {
            *visible = Visibility::Visible;
        } else {
            *visible = Visibility::Hidden;
        }
    }
    if let Ok(mut visible) = visibility.get_mut(ui_state.retire_text) {
        if *game_state == GameState::Retire {
            *visible = Visibility::Visible;
        } else {
            *visible = Visibility::Hidden;
        }
    }
}
