use crate::prelude::*;

#[derive(Component)]
pub struct CargoShip {
    aggressed: bool,
    sections_health: [f32; 8],
    sections_destroyed: [bool; 8],
}

impl CargoShip {
    pub fn new() -> Self {
        Self {
            aggressed: false,
            sections_health: [100.0; 8],
            sections_destroyed: [false; 8],
        }
    }

    pub fn damage_section(&mut self, section: usize, damage: f32) {
        self.aggressed = true;
        self.sections_health[section] -= damage;
    }

    pub fn section_must_die(&self, section: usize) -> bool {
        self.sections_health[section] <= 0.0 && !self.sections_destroyed[section]
    }

    pub fn section_alive(&self, section: usize) -> bool {
        self.sections_destroyed[section]
    }
}

#[derive(Component, Debug)]
pub struct CargoSection {
    pub index: usize,
    pub skeleton_bone: &'static str,
    pub hit_animation: &'static str,
}

const CARGO_SHIP_THRUST: f32 = 80000.0;
const CARGO_SECTION_MASS: f32 = 1000.0;
const CARGO_SHIP_MASS: f32 = CARGO_SECTION_MASS * 8.0 + 2000.0;
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

pub fn spawn_cargo_ship(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    skeletons: Res<Skeletons>,
) {
    let (indicator, indicator_text) = create_indicator_with_text(&mut commands, &game_assets);
    commands
        .spawn((
            SpineBundle {
                skeleton: skeletons.cargo_ship.clone(),
                ..Default::default()
            },
            InertiaVolume::new(CARGO_SHIP_MASS, 0.0),
            DistantIndicator::new_local(indicator, indicator_text),
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

pub fn cargo_ship_jet_animation_system(mut players: Query<(&CargoShip, &mut Spine)>) {
    for (cargo_ship, mut spine) in players.iter_mut() {
        let Spine(SkeletonController { skeleton, .. }) = &mut *spine;
        if let Some(left_jet) = skeleton.find_slot_mut("left_jet") {
            toggle_jet(left_jet, cargo_ship.aggressed);
        }
        if let Some(right_jet) = skeleton.find_slot_mut("right_jet") {
            toggle_jet(right_jet, cargo_ship.aggressed);
        }
    }
}

pub fn cargo_ship_escape_system(
    time: Res<Time>,
    mut cargo_ships: Query<(&CargoShip, &mut InertiaVolume)>,
) {
    let dt = time.delta_seconds();
    for (cargo_ship, mut inertia) in cargo_ships.iter_mut() {
        if cargo_ship.aggressed {
            inertia.apply_thrust_force(CARGO_SHIP_THRUST, dt);
        }
    }
}

pub fn cargo_ship_drop_system(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut cargo_ships: Query<(Entity, &CargoShip, &mut InertiaVolume, &mut Spine)>,
    cargo_sections: Query<(Entity, &CargoSection, &Parent, &GlobalTransform)>,
) {
    for (ship_entity, cargo_ship, mut ship_inertia, mut cargo_skeleton) in cargo_ships.iter_mut() {
        for section_idx in 0..8 {
            if cargo_ship.section_must_die(section_idx) {
                // Pinata!
                if let Some((ship_section, section, _parent, transform)) = cargo_sections
                    .iter()
                    .find(|(_ship_section, section, parent, _)| {
                        parent.get() == ship_entity && section.index == section_idx
                    })
                {
                    for _ in 0..10 {
                        spawn_salvage(
                            transform.translation().x,
                            transform.translation().y,
                            ship_inertia.velocity
                                + Quat::from_rotation_z(rand::random::<f32>() * PI * 2.)
                                    .mul_vec3(Vec3::X)
                                    .truncate()
                                    * (rand::random::<f32>() * 100.0),
                            &mut commands,
                            game_assets.salvage.clone(),
                            rand::random::<f32>() * 20.0 + 10.0,
                        );
                    }
                    ship_inertia.mass -= CARGO_SECTION_MASS;
                    commands.entity(ship_section).despawn();
                    if let Some(mut section_bone) =
                        cargo_skeleton.skeleton.find_bone_mut(section.skeleton_bone)
                    {
                        // Make the section disappear!
                        section_bone.set_scale_x(0.);
                    }
                }
            }
        }
    }
}