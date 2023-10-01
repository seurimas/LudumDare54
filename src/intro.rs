use crate::{home::Career, prelude::*, ui::UiState};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_intro_text.run_if(in_state(GameState::Playing)),),
        );
    }
}

fn update_intro_text(
    mut ui_state: ResMut<UiState>,
    career: Res<Career>,
    mut texts: Query<&mut Text>,
    mut visibility: Query<&mut Visibility>,
) {
    if career.intro_complete {
        return;
    }
    if let Ok(mut text) = texts.get_mut(ui_state.central_text) {}
}
