use crate::prelude::*;

#[derive(PartialEq, Debug)]
enum CargoShipEscape {
    Passive,
    Jammed,
    Jumping { progress: f32 },
    Jumped { progress: f32 },
}

#[derive(Component)]
pub struct CargoShip {
    pub value_modifier: f32,
    pub aggressed: bool,
    sections_health: [f32; 8],
    pub sections_destroyed: [bool; 8],
    fire_speed: f32,
    turret_cooldowns: [f32; 2],
    escape_state: CargoShipEscape,
    jump_time: f32,
}

const CARGO_SHIP_SECTION_HEALTH: f32 = 75.0;

impl CargoShip {
    pub fn new(value_modifier: f32) -> Self {
        Self {
            value_modifier,
            aggressed: false,
            sections_health: [CARGO_SHIP_SECTION_HEALTH; 8],
            sections_destroyed: [false; 8],
            fire_speed: 1.,
            turret_cooldowns: [0.0; 2],
            escape_state: CargoShipEscape::Passive,
            jump_time: 3.0,
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
        !self.sections_destroyed[section]
    }

    pub fn section_damaged(&self, section: usize) -> Option<usize> {
        if self.sections_health[section] <= CARGO_SHIP_SECTION_HEALTH * 0.15
            && !self.sections_destroyed[section]
        {
            Some(2)
        } else if self.sections_health[section] <= CARGO_SHIP_SECTION_HEALTH * 0.50
            && !self.sections_destroyed[section]
        {
            Some(1)
        } else if self.sections_health[section] <= CARGO_SHIP_SECTION_HEALTH * 0.90
            && !self.sections_destroyed[section]
        {
            Some(0)
        } else {
            None
        }
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
const SECTION_DAMAGE_SLOTS: [&'static str; 8] = [
    "cargo0_damage",
    "cargo1_damage",
    "cargo2_damage",
    "cargo3_damage",
    "cargo4_damage",
    "cargo5_damage",
    "cargo6_damage",
    "cargo7_damage",
];
const DAMAGE_ATTACHMENTS: [&'static str; 3] = ["Damage0", "Damage1", "Damage2"];
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

pub fn spawn_cargo_ships(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    skeletons: Res<Skeletons>,
    count: usize,
) {
    let angle = rand::thread_rng().gen_range(0.0..PI * 2.0);
    let direction = Vec2::new(f32::cos(angle), f32::sin(angle));
    let transform = Transform::from_xyz(
        direction.x * ARENA_SIZE * 2.,
        direction.y * ARENA_SIZE * 2.,
        0.,
    );
    let mut inertia = InertiaVolume::new(CARGO_SHIP_MASS, 0.0);
    inertia.velocity = -direction * 20.0;
    inertia.set_rotation(-angle);
    for idx in 0..count {
        let mut my_transform = transform.clone();
        if idx == 1 {
            my_transform.translation.x += direction.y * 300.0;
            my_transform.translation.y -= direction.x * 300.0;
        } else if idx == 2 {
            my_transform.translation.x -= direction.y * 300.0;
            my_transform.translation.y += direction.x * 300.0;
        }
        println!("Spawning cargo ship at {:?}", my_transform.translation);
        let (indicator, indicator_text) =
            create_indicator_with_text(&mut commands, &game_assets, true);
        commands
            .spawn((
                SpineBundle {
                    transform: my_transform,
                    skeleton: skeletons.cargo_ship.clone(),
                    ..Default::default()
                },
                inertia.clone(),
                DistantIndicator::new_local(indicator, indicator_text),
                CargoShip::new(count as f32 * 0.5 + 0.5),
                Regional,
                Jammable,
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

pub fn cargo_ship_drop_system(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut cargo_ships: Query<(Entity, &mut CargoShip, &mut InertiaVolume, &mut Spine)>,
    cargo_sections: Query<(Entity, &CargoSection, &Parent, &GlobalTransform)>,
) {
    for (ship_entity, mut cargo_ship, mut ship_inertia, mut cargo_skeleton) in
        cargo_ships.iter_mut()
    {
        for section_idx in 0..8 {
            if cargo_ship.section_must_die(section_idx) {
                cargo_ship.sections_destroyed[section_idx] = true;
                commands.spawn(AudioBundle {
                    source: game_assets.cargo_ship_section_destroyed.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
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
                            rand::random::<f32>() * 1.0 + 2.0,
                            (rand::random::<f32>() * 20.0 + 10.0) * cargo_ship.value_modifier,
                        );
                    }
                    spawn_upgrade(
                        transform.translation().x,
                        transform.translation().y,
                        ship_inertia.velocity
                            + Quat::from_rotation_z(rand::random::<f32>() * PI * 2.)
                                .mul_vec3(Vec3::X)
                                .truncate()
                                * (rand::random::<f32>() * 50.0),
                        &mut commands,
                        game_assets.upgrades.clone(),
                        Upgrade::random(),
                    );
                    ship_inertia.mass -= CARGO_SECTION_MASS;
                    commands.entity(ship_section).despawn();
                    if let Some(mut section_bone) =
                        cargo_skeleton.skeleton.find_bone_mut(section.skeleton_bone)
                    {
                        // Make the section disappear!
                        section_bone.set_scale_x(0.);
                    }
                }
            } else if cargo_ship.section_alive(section_idx) {
                let attachment =
                    cargo_ship
                        .section_damaged(section_idx)
                        .and_then(|damage_amount| {
                            cargo_skeleton.skeleton.get_attachment_for_slot_name(
                                SECTION_DAMAGE_SLOTS[section_idx],
                                DAMAGE_ATTACHMENTS[damage_amount],
                            )
                        });
                if let Some(mut slot) = cargo_skeleton
                    .skeleton
                    .find_slot_mut(SECTION_DAMAGE_SLOTS[section_idx])
                {
                    unsafe {
                        slot.set_attachment(attachment);
                    }
                }
            }
        }
    }
}

const CARGO_SHIP_LASER_SPEED: f32 = 500.0;
const CARGO_SHIP_LASER_DISTANCE_SQ: f32 = 300.0 * 300.0;

pub fn cargo_ship_escape_system(
    time: Res<Time>,
    mut cargo_ships: Query<(
        Entity,
        &mut CargoShip,
        &mut InertiaVolume,
        &DistantIndicator,
        Option<&Jammed>,
    )>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let dt = time.delta_seconds();
    let someone_aggressed = cargo_ships
        .iter_mut()
        .any(|(_, cargo_ship, _, _, _)| cargo_ship.aggressed);
    for (cargo_entity, mut cargo_ship, mut inertia, indicators, m_jammed) in cargo_ships.iter_mut()
    {
        if cargo_ship.aggressed && cargo_ship.escape_state == CargoShipEscape::Passive {
            cargo_ship.escape_state = if m_jammed.is_some() {
                CargoShipEscape::Jammed
            } else {
                CargoShipEscape::Jumping { progress: 0.0 }
            };
        } else if !cargo_ship.aggressed && (someone_aggressed || m_jammed.is_some()) {
            cargo_ship.aggressed = true;
        }
        match cargo_ship.escape_state {
            CargoShipEscape::Jumping { progress } => {
                if m_jammed.is_some() {
                    cargo_ship.escape_state = CargoShipEscape::Jammed;
                } else {
                    inertia.apply_thrust_force(CARGO_SHIP_THRUST, dt);
                    if progress > cargo_ship.jump_time {
                        commands.spawn(AudioBundle {
                            source: game_assets.cargo_ship_hyperdrive.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                        cargo_ship.escape_state = CargoShipEscape::Jumped { progress: 0.0 };
                    } else {
                        cargo_ship.escape_state = CargoShipEscape::Jumping {
                            progress: progress + dt,
                        };
                    }
                }
            }
            CargoShipEscape::Jumped { progress } => {
                inertia.set_forward_speed(HYPERDRIVE_SPEED * 2.0);
                if progress > 1. {
                    commands.entity(cargo_entity).despawn_recursive();
                    commands.entity(indicators.get_indicator()).despawn();
                    commands.entity(indicators.get_indicator_text()).despawn();
                } else {
                    cargo_ship.escape_state = CargoShipEscape::Jumped {
                        progress: progress + dt,
                    };
                }
            }
            CargoShipEscape::Jammed => {
                if m_jammed.is_none() {
                    cargo_ship.escape_state = CargoShipEscape::Jumping { progress: 0.0 };
                } else {
                    inertia.apply_thrust_force(CARGO_SHIP_THRUST, dt);
                }
            }
            CargoShipEscape::Passive => {
                // Much ado about nothing.
            }
        }
    }
}

pub fn cargo_ship_defense_system(
    time: Res<Time>,
    players: Query<(&Player, &Transform, &InertiaVolume)>,
    mut cargo_ships: Query<(&mut CargoShip, &Transform, &InertiaVolume, &mut Spine)>,
    mut commands: Commands,
    lasers: Res<Lasers>,
    game_assets: Res<GameAssets>,
) {
    if players.is_empty() {
        return;
    }
    let player_position = players.single().1.translation;
    let player_velocity = players.single().2.velocity;
    for (mut cargo_ship, location, inertia, mut spine) in cargo_ships.iter_mut() {
        if !cargo_ship.aggressed {
            continue;
        }
        let relative_velocity = player_velocity - inertia.velocity;
        for (turret_idx, turret_name) in ["forward_turret", "rear_turret"].iter().enumerate() {
            cargo_ship.turret_cooldowns[turret_idx] -= time.delta_seconds();
            let local_turret_location = get_turret_location(&spine, turret_name);
            let turret_location = location
                .transform_point(local_turret_location.extend(0.0))
                .truncate();
            let delta = player_position.truncate() - turret_location;
            if delta.length_squared() > CARGO_SHIP_LASER_DISTANCE_SQ {
                // Too far away to shoot
                continue;
            }
            if let Some(target_location) = aim_ahead_location(
                turret_location,
                delta,
                relative_velocity,
                CARGO_SHIP_LASER_SPEED,
            ) {
                rotate_towards_world_location(
                    &mut spine,
                    turret_name,
                    location,
                    target_location,
                    inertia,
                );
                if cargo_ship.turret_cooldowns[turret_idx] <= 0.0 {
                    cargo_ship.turret_cooldowns[turret_idx] = cargo_ship.fire_speed;
                    commands.spawn(AudioBundle {
                        source: game_assets.cargo_ship_laser.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                    fire_laser_from_turret(
                        turret_name,
                        &spine,
                        location,
                        inertia,
                        &mut commands,
                        lasers.cargo_ship_laser_mesh.clone().into(),
                        lasers.cargo_ship_laser_material.clone(),
                        Bullet::Enemy,
                    );
                }
            } else {
                // Can't hit the player currently, aim towards them anyways
                rotate_towards_world_location(
                    &mut spine,
                    turret_name,
                    location,
                    player_position.truncate(),
                    inertia,
                );
            }
        }
    }
}
