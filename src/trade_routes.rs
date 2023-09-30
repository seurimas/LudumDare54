use crate::prelude::*;

pub struct TradeRoutesPlugin;

impl Plugin for TradeRoutesPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(OnEnter(GameState::Playing), spawn_cargo_ship)
            .add_systems(Update, (cargo_ship_jet_animation_system,));
    }
}

#[derive(Component)]
pub struct CargoShip {
    sections_health: [f32; 8],
}

impl CargoShip {
    pub fn new() -> Self {
        Self {
            sections_health: [100.0; 8],
        }
    }

    pub fn damage_section(&mut self, section: usize, damage: f32) {
        self.sections_health[section] -= damage;
    }

    pub fn section_alive(&self, section: usize) -> bool {
        self.sections_health[section] > 0.0
    }
}

#[derive(Component, Debug)]
pub struct CargoSection {
    pub index: usize,
    pub skeleton_bone: &'static str,
    pub hit_animation: &'static str,
}

const SECTION_BONES: [&'static str; 8] = [
    "cargo0", "cargo1", "cargo2", "cargo3", "cargo4", "cargo5", "cargo6", "cargo7",
];
const SECTION_HIT_ANIMATIONS: [&'static str; 8] = [
    "jiggle0", "jiggle1", "jiggle2", "jiggle3", "jiggle4", "jiggle5", "jiggle6", "jiggle7",
];
const SECTION_OFFSETS: [(f32, f32); 8] = [
    (-144., -16.),
    (-80., -16.),
    (-16., -16.),
    (48., -16.),
    (-144., 16.),
    (-80., 16.),
    (-16., 16.),
    (48., 16.),
];

impl CargoSection {
    pub fn bundle(index: usize) -> (Transform, GlobalTransform, Self, InertiaVolume) {
        (
            Transform::from_xyz(SECTION_OFFSETS[index].0, SECTION_OFFSETS[index].1, 0.),
            GlobalTransform::default(),
            CargoSection {
                index,
                skeleton_bone: SECTION_BONES[index],
                hit_animation: SECTION_HIT_ANIMATIONS[index],
            },
            InertiaVolume::new(1.0, 32.0),
        )
    }
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

    commands
        .spawn((
            SpineBundle {
                skeleton: skeletons.cargo_ship.clone(),
                ..Default::default()
            },
            InertiaVolume::new(1.0, 1.0),
            DistantIndicator {
                indicator,
                indicator_text,
            },
            CargoShip::new(),
        ))
        .with_children(|parent| {
            // Spawn all 8 cargo sections.
            parent.spawn((CargoSection::bundle(0),));
            parent.spawn((CargoSection::bundle(1),));
            parent.spawn((CargoSection::bundle(2),));
            parent.spawn((CargoSection::bundle(3),));
            parent.spawn((CargoSection::bundle(4),));
            parent.spawn((CargoSection::bundle(5),));
            parent.spawn((CargoSection::bundle(6),));
            parent.spawn((CargoSection::bundle(7),));
        });
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
