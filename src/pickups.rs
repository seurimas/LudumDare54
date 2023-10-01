use std::fmt::{Display, Formatter};

use bevy::ecs::system::EntityCommands;

use crate::prelude::*;

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_pickup_system.run_if(not(in_state(GameState::Loading))),),
        );
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Upgrade {
    EngineUpgrade,
    ShieldRecharge,
    ShieldStrength,
    HullStrength,
    FireSpeed,
    // FirePower,
    JammerRange,
    JammerEfficiency,
}

impl Display for Upgrade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Upgrade::EngineUpgrade => write!(f, "Engine Upgrade"),
            Upgrade::ShieldRecharge => write!(f, "Shield Recharge"),
            Upgrade::ShieldStrength => write!(f, "Shield Strength"),
            Upgrade::HullStrength => write!(f, "Hull Strength"),
            Upgrade::FireSpeed => write!(f, "Fire Speed"),
            // Upgrade::FirePower => write!(f, "Fire Power"),
            Upgrade::JammerRange => write!(f, "Jammer Range"),
            Upgrade::JammerEfficiency => write!(f, "Jammer Efficiency"),
        }
    }
}

impl Upgrade {
    pub fn get_upgrade_material_name(&self) -> String {
        match self {
            Upgrade::EngineUpgrade => "XM Engine Coils",
            Upgrade::ShieldRecharge => "XM Shield Generators",
            Upgrade::ShieldStrength => "XM Shield Capacitors",
            Upgrade::HullStrength => "XM Plates",
            Upgrade::FireSpeed => "XM Plasma Injectors",
            // Upgrade::FirePower => "Fire Power Material",
            Upgrade::JammerRange => "XM Attenuators",
            Upgrade::JammerEfficiency => "XM Amplifiers",
        }
        .to_string()
    }

    pub fn get_sprite_index(&self) -> usize {
        match self {
            Upgrade::EngineUpgrade => 0,
            Upgrade::ShieldRecharge => 1,
            Upgrade::ShieldStrength => 2,
            Upgrade::HullStrength => 3,
            Upgrade::FireSpeed => 4,
            // Upgrade::FirePower => 5,
            Upgrade::JammerRange => 6,
            Upgrade::JammerEfficiency => 7,
        }
    }

    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..7) {
            0 => Upgrade::EngineUpgrade,
            1 => Upgrade::ShieldRecharge,
            2 => Upgrade::ShieldStrength,
            3 => Upgrade::HullStrength,
            4 => Upgrade::FireSpeed,
            // 5 => Upgrade::FirePower,
            5 => Upgrade::JammerRange,
            6 => Upgrade::JammerEfficiency,
            _ => panic!("Invalid upgrade index!"),
        }
    }
}

#[derive(Component)]
pub enum Pickup {
    ExoticMaterial(f32),
    Salvage { mass: f32, value: f32 },
    Upgrade { mass: f32, upgrade: Upgrade },
}

pub fn spawn_exotic<'w, 's, 'a>(
    x: f32,
    y: f32,
    mut commands: &'a mut Commands<'w, 's>,
    texture: Handle<Image>,
    value: f32,
) -> EntityCommands<'w, 's, 'a> {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture,
            sprite: Sprite {
                color: Color::rgba(10., 10., 0., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        InertiaVolume::new(1.0, 16.0),
        Pickup::ExoticMaterial(value),
        Regional,
        Jammer {
            radius: rand::thread_rng().gen_range((value * 100.)..(value * 150.)),
            progress: 0.0,
        },
    ))
}

pub fn spawn_salvage(
    x: f32,
    y: f32,
    velocity: Vec2,
    mut commands: &mut Commands<'_, '_>,
    texture: Handle<Image>,
    mass: f32,
    value: f32,
) {
    let mut inertia_volume = InertiaVolume::new(1.0, 8.0);
    inertia_volume.velocity = velocity;
    inertia_volume.rotation_velocity = 0.1;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture,
            sprite: Sprite {
                color: Color::rgba(value / 10., value / 10., value / 10., value / 10.),
                ..Default::default()
            },
            ..Default::default()
        },
        Regional,
        inertia_volume,
        Pickup::Salvage { mass, value },
    ));
}

pub fn spawn_upgrade(
    x: f32,
    y: f32,
    velocity: Vec2,
    mut commands: &mut Commands<'_, '_>,
    texture_atlas: Handle<TextureAtlas>,
    upgrade: Upgrade,
) {
    let mut inertia_volume = InertiaVolume::new(1.0, 8.0);
    inertia_volume.velocity = velocity;
    inertia_volume.rotation_velocity = 0.1;
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture_atlas,
            sprite: TextureAtlasSprite {
                index: upgrade.get_sprite_index(),
                color: Color::rgba(8., 8., 4., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        Regional,
        inertia_volume,
        Pickup::Upgrade { mass: 10., upgrade },
    ));
}

fn player_pickup_system(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    mut players: Query<&mut Player>,
    pickups: Query<&Pickup>,
    game_assets: Res<GameAssets>,
) {
    for collision in collisions.iter() {
        if let Ok(mut player) = players.get_mut(collision.e0) {
            if let Ok(pickup) = pickups.get(collision.e1) {
                match pickup {
                    Pickup::ExoticMaterial(amount) => {
                        if *amount > player.cargo_space_left() {
                            continue;
                        }
                        commands.spawn(AudioBundle {
                            source: game_assets.pickup_xm.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                        player.exotic_material += amount.min(player.cargo_space_left());
                    }
                    Pickup::Salvage { mass, value } => {
                        if *mass > player.cargo_space_left() {
                            continue;
                        }
                        commands.spawn(AudioBundle {
                            source: game_assets.pickup.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                        player.salvage_mass += mass;
                        player.salvage_value += value;
                    }
                    Pickup::Upgrade { mass, upgrade } => {
                        if *mass > player.cargo_space_left() {
                            continue;
                        }
                        commands.spawn(AudioBundle {
                            source: game_assets.upgrade.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                        player.upgrade_mass += mass;
                        player.upgrade_materials.push(*upgrade);
                    }
                }
                if let Some(mut pickup_entity) = commands.get_entity(collision.e1) {
                    pickup_entity.despawn();
                }
            }
        }
    }
}
