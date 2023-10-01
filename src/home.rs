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
    pub last_upgrades: Vec<Upgrade>,
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

fn handle_go_home(
    mut players: Query<&mut Player>,
    mut career: ResMut<Career>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let mut player = players.single_mut();
    career.earnings += player.salvage_value;
    player.salvage_mass = 0.0;
    player.upgrade_mass = 0.0;
    player.salvage_value = 0.0;

    let repair_costs = player.get_repair_cost().min(career.earnings);
    career.earnings -= repair_costs;
    career.last_repair_costs = repair_costs;
    player.repair(repair_costs);

    career.last_upgrades.clear();
    if !player.upgrade_materials.is_empty() {
        commands.spawn(AudioBundle {
            source: game_assets.upgrade.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
        for upgrade in player.upgrade_materials.drain(..).collect::<Vec<_>>() {
            player.apply_upgrade(upgrade);
            career.last_upgrades.push(upgrade);
        }
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

            if career.last_upgrades.len() > 0 {
                text.sections[4].value = "\n".to_string();
                for upgrade in career.last_upgrades.iter() {
                    text.sections[4].value =
                        format!("{}\nUpgrade found: {}", text.sections[4].value, upgrade);
                }
            } else {
                text.sections[4].value = "\n".to_string();
            }
            text.sections[5].value = format!("\n\n{} days left", 10 - career.days_survived);
        }
    } else {
        if let Ok(mut visible) = visibility.get_mut(ui_state.home_text) {
            *visible = Visibility::Hidden;
        }
    }
}
