use crate::{home::Career, prelude::*, trade_routes::spawn_starting_system, ui::UiState};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_intro_text.run_if(in_state(GameState::Playing)),),
        );
    }
}

pub const INTRO_STAGES: usize = 20;

fn update_intro_text(
    mut ui_state: ResMut<UiState>,
    mut career: ResMut<Career>,
    mut texts: Query<&mut Text>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    players: Query<(&Player, &Transform)>,
    pickups: Query<&Pickup>,
    jammers: Query<&Jammer>,
) {
    if career.intro_complete() {
        return;
    }
    if let Ok(mut text) = texts.get_mut(ui_state.central_text) {
        text.sections[0].value = match career.intro_stage {
            0 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("Welcome! You are the captain of this fine vessel.\n[W], [S] to engage main thrusters.\n[Q], [E], [SHIFT] will engage maneuvering thrusting.\n\nPress [Space] to continue.")
            }
            1 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("You are a pirate. Your goal is to steal as much as you can.\n\nPress [Space] to continue.")
            }
            2 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("You have two advantages over your prey: speed and hyperdrive jammers.\n\nPress [Space] to continue.")
            }
            3 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("The cargo ships you will encounter are slow and their turrets are weak.\n\nPress [Space] to continue.")
            }
            4 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("However, they are capable of emergency hyperdrive jumps.\n\nPress [Space] to continue.")
            }
            5 => {
                let (indicator, indicator_text) =
                    create_indicator_with_text(&mut commands, &game_assets, true);
                career.intro_stage += 1;
                let x = players.single().1.translation.x;
                let y = players.single().1.translation.y;
                let x = x + 200.;
                let y = y + 200.;
                spawn_exotic(x, y, &mut commands, game_assets.exotic.clone(), 20.)
                    .insert(DistantIndicator::new_local(indicator, indicator_text));
                format!("Fly into the nearby XM asteroid to pick up some exotic matter.")
            }
            6 => {
                if pickups.iter().next().is_none() {
                    career.intro_stage += 1;
                }
                format!("Fly into the nearby XM asteroid to pick up some exotic matter.")
            }
            7 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("Salvage, exotic matter, and upgrades will be added to your cargo, but you have limited space.\n\nPress [Space] to continue.")
            }
            8 => {
                if jammers.iter().next().is_some() {
                    career.intro_stage += 1;
                }
                format!("You can use exotic matter to deploy hyperdrive jammers.\n\nPress [G] to deploy a jammer.")
            }
            9 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!(
                    "Jammers will prevent ships from jumping away.\n\nPress [Space] to continue."
                )
            }
            10 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("When you have a cargo ship trapped, shoot it with your lasers.\n[LEFT CLICK] to shoot.\n\nPress [Space] to continue.")
            }
            11 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("Cargo ships will try to escape the jamming field.\nDeploy more jammers to prevent their escape.\n\nPress [Space] to continue.")
            }
            12 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("You have 10 days to make as much as you can before the authorities extend their patrols.\n\nPress [Space] to continue.")
            }
            13 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("Visit your hideout to end the day.\nThere, you'll buy repairs and install all upgrades.\n\nPress [Space] to continue.")
            }
            14 => {
                if keys.just_pressed(KeyCode::Space) {
                    career.intro_stage += 1;
                }
                format!("You are ready to begin your career.\nHyperjump to your first target!\n\nPress [Space] to continue.")
            }
            _ => {
                career.intro_stage = INTRO_STAGES + 10;
                spawn_starting_system(commands, game_assets);
                format!("")
            }
        }
    }
}
