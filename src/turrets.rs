use std::f32::consts::PI;

use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::prelude::*;

pub fn get_turret_location(spine: &Spine, turret_name: &'static str) -> Vec2 {
    if let Some(bone) = spine.skeleton.find_bone(turret_name) {
        // "world" is relative to the skeleton, not the whole world.
        bone.world_position().into()
    } else {
        Vec2::ZERO
    }
}

pub fn get_turret_rotation(spine: &Spine, turret_name: &'static str) -> f32 {
    if let Some(bone) = spine.skeleton.find_bone(turret_name) {
        bone.rotation() * PI / 180.0
    } else {
        0.0
    }
}

pub fn rotate_turret(spine: &mut Spine, turret_name: &'static str, rotation: f32) {
    if let Some(mut bone) = spine.skeleton.find_bone_mut(turret_name) {
        bone.set_rotation(rotation * 180.0 / PI);
    }
}

pub fn fire_laser_from_turret(
    turret_name: &'static str,
    spine: &Spine,
    location: &Transform,
    my_inertia: &InertiaVolume,
    commands: &mut Commands<'_, '_>,
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
) {
    // Build a transform for the bullet.
    let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
    // Shoot from the appropriate turret.
    let local_turret_location = get_turret_location(&spine, turret_name);
    transform.translation = location.transform_point(local_turret_location.extend(10.0));
    // Shoot in the appropriate direction.
    let turret_rotation = get_turret_rotation(&spine, turret_name);
    let total_rotation = turret_rotation + my_inertia.rotation();
    let direction = Vec2::new(f32::cos(total_rotation), f32::sin(total_rotation));
    transform.rotation = Quat::from_rotation_z(total_rotation);
    // Give it some speed!
    let mut inertia = InertiaVolume::new(1.0, 1.0);
    inertia.velocity = my_inertia.velocity + direction * 1000.0;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        },
        inertia,
        Bullet::Player,
    ));
}
