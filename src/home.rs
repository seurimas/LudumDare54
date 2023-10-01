use crate::{prelude::*, ui::UiState};

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Career>()
            .add_systems(OnEnter(GameState::Home), handle_go_home)
            .add_systems(
                Update,
                (
                    update_home_ui.run_if(not(in_state(GameState::Loading))),
                    handle_home_input.run_if(in_state(GameState::Home)),
                ),
            );
    }
}

#[derive(Resource, Default)]
pub struct Career {
    pub earnings: f32,
    pub last_repair_costs: f32,
    pub last_upgrade: Option<Upgrade>,
    pub days_survived: u32,
}

#[derive(Component)]
pub struct HomeInSystem;

pub fn spawn_home_in_system(
    location: Vec2,
    game_assets: &Res<GameAssets>,
    mut commands: &mut Commands,
) {
    let (indicator, indicator_text) = create_indicator_with_text(commands, game_assets, false);
    commands.spawn((
        TransformBundle::default(),
        SystemLocation { location },
        HomeInSystem,
        DistantIndicator::new_system(indicator, indicator_text, Vec2::ZERO),
    ));
}

fn handle_go_home(mut players: Query<&mut Player>, mut career: ResMut<Career>) {
    let mut player = players.single_mut();
    career.earnings += player.salvage_value;
    player.salvage_mass = 0.0;
    player.salvage_value = 0.0;

    let repair_costs = player.get_repair_cost().min(career.earnings);
    career.earnings -= repair_costs;
    player.repair(repair_costs);

    if let Some(upgrade) = player.upgrade_material {
        player.apply_upgrade(upgrade);
        career.last_upgrade = Some(upgrade);
    } else {
        career.last_upgrade = None;
    }

    career.days_survived += 1;
}

fn handle_home_input(mut next_state: ResMut<NextState<GameState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn update_home_ui(
    game_state: Res<State<GameState>>,
    mut ui_state: ResMut<UiState>,
    career: Res<Career>,
    mut texts: Query<&mut Text>,
    mut visibility: Query<&mut Visibility>,
) {
    if *game_state == GameState::Home {
        if let Ok(mut visible) = visibility.get_mut(ui_state.home_text) {
            *visible = Visibility::Visible;
        }
        if let Ok(mut text) = texts.get_mut(ui_state.home_text) {
            text.sections[1].value = format!("${:.0}", career.earnings);
            text.sections[3].value = format!("${:.0}", career.last_repair_costs);
            if let Some(upgrade) = &career.last_upgrade {
                text.sections[4].value = "Upgrade found: ".to_string();
                text.sections[5].value = format!("{}", upgrade);
            } else {
                text.sections[4].value = "\n".to_string();
                text.sections[5].value = "".to_string();
            }
            text.sections[6].value = format!("\n{} days left", 10 - career.days_survived);
        }
    } else {
        if let Ok(mut visible) = visibility.get_mut(ui_state.home_text) {
            *visible = Visibility::Hidden;
        }
    }
}
