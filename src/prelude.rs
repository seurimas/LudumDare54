pub use crate::assets::{GameAssets, Lasers, Skeletons};
pub use crate::bullets::Bullet;
pub use crate::game_state::GameState;
pub use crate::indicators::{create_indicator_with_text, DistantIndicator};
pub use crate::jamming::{Jammable, Jammed, Jammer};
pub use crate::physics::{Collision, InertiaVolume};
pub use crate::pickups::{spawn_exotic, spawn_salvage, spawn_upgrade, Pickup, Upgrade};
pub use crate::player::Player;
pub use crate::space_pixels::SpacePixel;
pub use crate::trade_routes::{
    CargoSection, CargoShip, Regional, SystemLocation, ARENA_SIZE, HYPERDRIVE_SPEED,
};
pub use crate::turrets::*;
use bevy::ecs::system::Command;
pub use bevy::prelude::*;
pub use bevy::utils::HashMap;
pub use bevy_spine::prelude::*;
pub use bevy_spine::rusty_spine::{c_interface::CTmpMut, Skeleton, Slot};
pub use bevy_spine::{SkeletonController, SpineBundle};
pub use rand::Rng;
pub use std::f32::consts::PI;

pub const AUDIO_SCALE: f32 = 200.0;

/// A [`Command`] that adds the components in a [`Bundle`] to an entity, and doesn't panic if it doesn't exist.
pub struct InsertSafe<T> {
    /// The entity to which the components will be added.
    pub entity: Entity,
    /// The [`Bundle`] containing the components that will be added to the entity.
    pub bundle: T,
}

impl<T> Command for InsertSafe<T>
where
    T: Bundle + 'static,
{
    fn apply(self, world: &mut World) {
        if let Some(mut entity) = world.get_entity_mut(self.entity) {
            entity.insert(self.bundle);
        } else {
            // Do nothing, we're fine.
        }
    }
}

pub fn aim_ahead_time(delta: Vec2, relative_velocity: Vec2, speed: f32) -> f32 {
    let a = relative_velocity.length_squared() - speed * speed;
    let b = 2.0 * relative_velocity.dot(delta);
    let c = delta.length_squared();
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        -1.0
    } else {
        let t = (-b - d.sqrt()) / (2.0 * a);
        if t < 0.0 {
            -1.0
        } else {
            t
        }
    }
}

pub fn aim_ahead_location(
    start: Vec2,
    delta: Vec2,
    relative_velocity: Vec2,
    speed: f32,
) -> Option<Vec2> {
    let t = aim_ahead_time(delta, relative_velocity, speed);
    if t < 0.0 {
        None
    } else {
        let target = start + delta + relative_velocity * t;
        Some(target)
    }
}

pub fn rotations_match(rotation1: f32, rotation2: f32, leeway: f32) -> bool {
    let rotation1 = rotation1 % (2.0 * PI);
    let rotation2 = rotation2 % (2.0 * PI);
    let diff = (rotation1 - rotation2).abs();
    diff < leeway
}

#[cfg(test)]
mod prelude_tests {
    use super::*;

    #[test]
    fn test_rotations_opposite() {
        assert_eq!(rotations_match(0.0, 0.0, PI / 4.), true);
        assert_eq!(rotations_match(PI, 0.0, PI / 4.), false);
    }
}
