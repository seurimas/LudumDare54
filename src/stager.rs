mod assets;
mod bullets;
mod game_over;
mod game_state;
mod home;
mod indicators;
mod intro;
mod jamming;
mod physics;
mod pickups;
mod player;
mod prelude;
mod space_pixels;
mod trade_routes;
mod turrets;
mod ui;

#[macro_use]
extern crate lazy_static;
use assets::GameAssetsPlugin;
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    text::DEFAULT_FONT_HANDLE,
    window::WindowResolution,
};
use bevy_spine::SpinePlugin;
use bullets::BulletsPlugin;
use game_over::GameOverPlugin;
use home::HomePlugin;
use indicators::IndicatorsPlugin;
use intro::IntroPlugin;
use jamming::{indicate_jamming_on_skeleton, JammingPlugin};
use physics::PhysicsPlugin;
use pickups::PickupsPlugin;
use player::{toggle_player_jet, PlayerPlugin};
use space_pixels::SpacePixelsPlugin;
use trade_routes::{toggle_cargo_jet, TradeRoutesPlugin, DAMAGE_ATTACHMENTS, SECTION_DAMAGE_SLOTS};
use ui::GameUiPlugin;

use crate::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Piracy 2444".to_string(),
                resolution: WindowResolution::new(948., 533.),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((SpinePlugin, GameAssetsPlugin, SpacePixelsPlugin))
        .add_systems(OnEnter(GameState::Playing), stage_cover)
        .add_systems(
            Update,
            (
                indicate_jamming_on_skeleton,
                animate_cover.run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
}

#[derive(Component)]
pub enum Staged {
    Player,
    Cargo,
    Bullet,
}

fn stage_cover(mut commands: Commands, assets: Res<GameAssets>, skeletons: Res<Skeletons>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            tonemapping: Tonemapping::Reinhard,
            ..default()
        },
        // BloomSettings::OLD_SCHOOL,
        BloomSettings::default(),
    ));

    let mut transform = Transform::from_translation(Vec3::new(-140.0, -90.0, 10.0));
    transform.rotation = Quat::from_rotation_z(PI / 4.);
    let mut inertia = InertiaVolume::new(0.0, 0.0);
    inertia.set_rotation(PI / 4.);
    commands.spawn((
        SpineBundle {
            skeleton: skeletons.player_ship.clone(),
            transform,
            ..Default::default()
        },
        Jammable,
        inertia,
        Staged::Player,
    ));

    let mut transform = Transform::from_translation(Vec3::new(200.0, 50.0, 10.0));
    transform.rotation = Quat::from_rotation_z(PI / 4.);
    let mut inertia = InertiaVolume::new(0.0, 0.0);
    inertia.set_rotation(PI / 4.);
    commands.spawn((
        SpineBundle {
            skeleton: skeletons.cargo_ship.clone(),
            transform,
            ..Default::default()
        },
        Jammable,
        Jammed,
        inertia,
        Staged::Cargo,
    ));

    let mut transform = Transform::from_translation(Vec3::new(-160.0, 100.0, 10.0));
    commands.spawn(
        (Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "Space ".to_string(),
                    style: TextStyle {
                        font: DEFAULT_FONT_HANDLE.typed(),
                        font_size: 32.0,
                        color: Color::rgba(0., 10., 10., 3.),
                    },
                },
                TextSection {
                    value: "Piracy ".to_string(),
                    style: TextStyle {
                        font: DEFAULT_FONT_HANDLE.typed(),
                        font_size: 32.0,
                        color: Color::rgba(0., 10., 10., 3.),
                    },
                },
                TextSection {
                    value: "2444".to_string(),
                    style: TextStyle {
                        font: DEFAULT_FONT_HANDLE.typed(),
                        font_size: 32.0,
                        color: Color::rgba(10., 10., 0., 10.),
                    },
                },
            ]),
            transform,
            ..Default::default()
        }),
    );
}

fn animate_cover(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Spine, &InertiaVolume, &Transform, &Staged)>,
    staged_bullets: Query<&Staged, Without<Spine>>,
    mut bullets: Query<
        (Entity, &mut Transform, &InertiaVolume),
        (Without<Staged>, Or<(With<Pickup>, With<Bullet>)>),
    >,
    lasers: Res<Lasers>,
    game_assets: Res<GameAssets>,
) {
    let cargo_ship_section = query
        .iter()
        .find(|(_, _, _, staged)| {
            if let Staged::Cargo = staged {
                true
            } else {
                false
            }
        })
        .map(|(_, _, transform, _)| transform.translation)
        .unwrap_or(Vec3::ZERO)
        .truncate();
    let player_ship_section = query
        .iter()
        .find(|(_, _, _, staged)| {
            if let Staged::Player = staged {
                true
            } else {
                false
            }
        })
        .map(|(_, _, transform, _)| transform.translation)
        .unwrap_or(Vec3::ZERO)
        .truncate();
    let bullets_fired = staged_bullets.iter().next().is_some();
    for (mut spine, my_inertia, transform, staged) in query.iter_mut() {
        match staged {
            Staged::Player => {
                rotate_towards_world_location(
                    &mut spine,
                    "forward_turret",
                    transform,
                    cargo_ship_section,
                    my_inertia,
                );
                rotate_towards_world_location(
                    &mut spine,
                    "left_turret",
                    transform,
                    cargo_ship_section,
                    my_inertia,
                );
                rotate_towards_world_location(
                    &mut spine,
                    "right_turret",
                    transform,
                    cargo_ship_section,
                    my_inertia,
                );
                if let Some(left_jet) = spine.skeleton.find_slot_mut("left_jet") {
                    toggle_player_jet(left_jet, true);
                }
                if let Some(right_jet) = spine.skeleton.find_slot_mut("right_jet") {
                    toggle_player_jet(right_jet, true);
                }
                if let Some(right_maneuver) = spine.skeleton.find_slot_mut("right_maneuver") {
                    toggle_player_jet(right_maneuver, true);
                }
                if let Some(left_maneuver) = spine.skeleton.find_slot_mut("left_maneuver") {
                    toggle_player_jet(left_maneuver, false);
                }
                if let Some(left_reverse_jet) = spine.skeleton.find_slot_mut("left_reverse_jet") {
                    toggle_player_jet(left_reverse_jet, false);
                }
                if let Some(right_reverse_jet) = spine.skeleton.find_slot_mut("right_reverse_jet") {
                    toggle_player_jet(right_reverse_jet, false);
                }
                if !bullets_fired {
                    fire_laser_from_turret(
                        "forward_turret",
                        &spine,
                        transform,
                        my_inertia,
                        &mut commands,
                        lasers.player_laser_mesh.clone().into(),
                        lasers.player_laser_material.clone(),
                        Bullet::Player,
                    );
                    fire_laser_from_turret(
                        "left_turret",
                        &spine,
                        transform,
                        my_inertia,
                        &mut commands,
                        lasers.player_laser_mesh.clone().into(),
                        lasers.player_laser_material.clone(),
                        Bullet::Player,
                    );
                    fire_laser_from_turret(
                        "right_turret",
                        &spine,
                        transform,
                        my_inertia,
                        &mut commands,
                        lasers.player_laser_mesh.clone().into(),
                        lasers.player_laser_material.clone(),
                        Bullet::Player,
                    );
                }
            }
            Staged::Cargo => {
                rotate_towards_world_location(
                    &mut spine,
                    "forward_turret",
                    transform,
                    player_ship_section,
                    my_inertia,
                );
                rotate_towards_world_location(
                    &mut spine,
                    "rear_turret",
                    transform,
                    player_ship_section,
                    my_inertia,
                );
                if let Some(left_jet) = spine.skeleton.find_slot_mut("left_jet") {
                    toggle_cargo_jet(left_jet, true);
                }
                if let Some(right_jet) = spine.skeleton.find_slot_mut("right_jet") {
                    toggle_cargo_jet(right_jet, true);
                }
                if let Some(mut section_bone) = spine.skeleton.find_bone_mut("cargo0") {
                    // Make the section disappear!
                    section_bone.set_scale_x(0.);
                }
                if let Some(mut section_bone) = spine.skeleton.find_bone_mut("cargo4") {
                    // Make the section disappear!
                    section_bone.set_scale_x(0.);
                }
                let attachment = spine
                    .skeleton
                    .get_attachment_for_slot_name(SECTION_DAMAGE_SLOTS[5], DAMAGE_ATTACHMENTS[2]);
                if let Some(mut slot) = spine.skeleton.find_slot_mut(SECTION_DAMAGE_SLOTS[5]) {
                    unsafe {
                        slot.set_attachment(attachment);
                    }
                }
                if !bullets_fired {
                    for _ in 0..2 {
                        let x = transform.translation.x
                            + spine.skeleton.find_bone("cargo0").unwrap().world_x();
                        let y = transform.translation.y
                            + spine.skeleton.find_bone("cargo0").unwrap().world_y();
                        spawn_salvage(
                            x,
                            y,
                            my_inertia.velocity
                                + Quat::from_rotation_z(rand::random::<f32>() * PI * 2.)
                                    .mul_vec3(Vec3::X)
                                    .truncate()
                                    * (rand::random::<f32>() * 300.0),
                            &mut commands,
                            game_assets.salvage.clone(),
                            rand::random::<f32>() * 1.0 + 2.0,
                            (rand::random::<f32>() * 20.0 + 10.0) * 2.,
                        );
                    }
                }
            }
            _ => {}
        }
    }
    for (entity, mut transform, my_inertia) in bullets.iter_mut() {
        println!(
            "Bullet: {:?} {:?}",
            transform.translation, my_inertia.velocity
        );
        transform.translation += (my_inertia.velocity * 0.2).extend(0.);
        commands
            .entity(entity)
            .remove::<Bullet>()
            .insert(Staged::Bullet);
    }
}
