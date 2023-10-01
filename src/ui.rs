use bevy::text::DEFAULT_FONT_HANDLE;

use crate::prelude::*;

const CARGO_CELL_COUNT: usize = 100;
const CARGO_CELL_ROWS: usize = 4;
const CARGO_CELL_COLUMNS: usize = CARGO_CELL_COUNT / CARGO_CELL_ROWS;
const CARGO_CELL_SIZE: f32 = 16.;

pub struct GameUiPlugin;

#[derive(Resource)]
struct UiState {
    shield_display: Entity,
    hull_display: Entity,
    cargo_display: Entity,
    cargo_text: Entity,
    cargo_cells: Vec<Entity>,
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
                width: Val::Px(200.),
                height: Val::Px(20.),
                ..Default::default()
            },
            background_color: Color::rgba(0., 0., 1., 1.0).into(),
            text: Text::from_section(
                "Shields: 100%",
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
    let hull_display = commands
        .spawn((TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(20.),
                width: Val::Px(200.),
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
                display: Display::Grid,
                grid_template_rows: RepeatedGridTrack::flex(CARGO_CELL_ROWS as u16, 1.),
                grid_template_columns: RepeatedGridTrack::flex(CARGO_CELL_COLUMNS as u16, 1.),
                left: Val::Px(0.),
                top: Val::Px(60.),
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
                            width: Val::Px(200.),
                            height: Val::Px(20.),
                            ..Default::default()
                        },
                        text: Text::from_section(
                            "Cargo: 0/0",
                            TextStyle {
                                font: DEFAULT_FONT_HANDLE.typed(),
                                font_size: 20.,
                                color: Color::WHITE,
                            },
                        )
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

    // Save everything to the resource.
    commands.insert_resource(UiState {
        shield_display,
        hull_display,
        cargo_display,
        cargo_text: cargo_text.unwrap(),
        cargo_cells,
    });
}

fn update_ui(
    mut ui_state: ResMut<UiState>,
    mut bg_color: Query<&mut BackgroundColor>,
    mut text: Query<&mut Text>,
    player: Query<&Player>,
) {
    let player = player.single();
    // Display health!
    if let Ok(mut shield_text) = text.get_mut(ui_state.shield_display) {
        shield_text.sections[0].value = format!(
            "Shields: {:.0}%",
            player.shields / player.max_shields * 100.
        );
    }
    if let Ok(mut hull_text) = text.get_mut(ui_state.hull_display) {
        hull_text.sections[0].value = format!("Hull: {:.0}%", player.hull / player.max_hull * 100.);
    }
    // End health.

    // Display cargo!
    const EXOTIC_COLOR: Color = Color::rgba(1.0, 1.0, 0.0, 1.0);
    const SALVAGE_COLOR: Color = Color::rgba(0.5, 0.5, 0.5, 1.0);
    let mut exotic_drawn = 0;
    let mut salvage_drawn = 0;
    let exotics = player.exotic_material.ceil() as i32;
    let salvage = player.salvage.floor() as i32;
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
    }
}
