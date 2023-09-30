use crate::prelude::*;

pub struct TradeRoutesPlugin;

impl Plugin for TradeRoutesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_cargo_ship)
            .add_systems(Update, (cargo_ship_jet_animation_system,));
    }
}

#[derive(Component)]
pub struct CargoShip;

#[derive(Component)]
pub struct CargoSection {
    // pub
}

fn spawn_cargo_ship(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    skeletons: Res<Skeletons>,
) {
    let indicator = commands
        .spawn(ImageBundle {
            image: UiImage::new(game_assets.indicator.clone()),
            style: Style {
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
            ..Default::default()
        })
        .id();

    commands.spawn((
        SpineBundle {
            skeleton: skeletons.cargo_ship.clone(),
            ..Default::default()
        },
        InertiaVolume::new(1.0, 1.0),
        DistantIndicator {
            indicator,
            indicator_text,
        },
        CargoShip,
    ));
}

const JET_GREENNESS: f32 = 24.0;
const JET_BRIGHTNESS: f32 = 10.0;

fn toggle_jet(mut jet: CTmpMut<Skeleton, Slot>, on: bool) {
    if on {
        jet.color_mut().a = JET_BRIGHTNESS;
        jet.color_mut().g = JET_GREENNESS;
    } else {
        jet.color_mut().a = 0.0;
    }
}

fn cargo_ship_jet_animation_system(mut players: Query<(&CargoShip, &mut Spine)>) {
    for (_cargo_ship, mut spine) in players.iter_mut() {
        let Spine(SkeletonController { skeleton, .. }) = &mut *spine;
        if let Some(left_jet) = skeleton.find_slot_mut("left_jet") {
            toggle_jet(left_jet, true);
        }
        if let Some(right_jet) = skeleton.find_slot_mut("right_jet") {
            toggle_jet(right_jet, true);
        }
    }
}
