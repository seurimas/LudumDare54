use crate::{home::Career, prelude::*, ui::UiState};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_player_death_system.run_if(in_state(GameState::Playing)),
                check_player_retire_system.run_if(not(in_state(GameState::Loading))),
                update_game_over_text.run_if(in_state(GameState::GameOver)),
                update_retirement_text.run_if(in_state(GameState::Retire)),
            ),
        );
    }
}

fn check_player_death_system(
    mut player: Query<(&Player, &mut Transform)>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if player.single().0.is_dead() && *current_game_state == GameState::Playing {
        next_game_state.set(GameState::GameOver);
        player.single_mut().1.scale = Vec3::ZERO;
    }
}

fn check_player_retire_system(
    career: Res<Career>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if *current_game_state == GameState::Playing || *current_game_state == GameState::Home {
        if career.days_survived == 10 {
            next_game_state.set(GameState::Retire);
        }
    }
}

fn update_game_over_text(ui_state: Res<UiState>, career: Res<Career>, mut texts: Query<&mut Text>) {
    if let Ok(mut text) = texts.get_mut(ui_state.game_over_text) {
        text.sections[1].value = format!(
            "\n\nYour career of {} days came to violent end.",
            career.days_survived
        );
        text.sections[2].value = format!(
            "\nAll ${:.0} of your ill-gotten goods were eventually recovered.",
            career.earnings
        );
    }
}

fn update_retirement_text(
    mut ui_state: ResMut<UiState>,
    career: Res<Career>,
    mut texts: Query<&mut Text>,
    mut visibility: Query<&mut Visibility>,
) {
    if let Ok(mut text) = texts.get_mut(ui_state.retire_text) {
        text.sections[1].value = format!(
            "As the authorities closed in, you escape with ${:.0}.\n\n",
            career.earnings
        );
        if career.earnings > 2000. {
            text.sections[2].value =
                format!("You retire to a life of luxury, and easily escape your pursuers.\n");
        } else if career.earnings > 1000. {
            text.sections[2].value = format!(
                "You retire to a life of comfort, hiding from the authorities for the rest of your days.\n"
            );
        } else if career.earnings > 500. {
            text.sections[2].value = format!(
                "Your efforts to retire are thwarted by a bad game of roulette.\nThe authorities catch up to you and you are sent to a penal colony."
            );
        } else {
            text.sections[2].value = format!(
                "Your meager winnings are quickly spent.\nThe authorities catch up to you and you are sent to a penal colony."
            );
        }
    }
}
