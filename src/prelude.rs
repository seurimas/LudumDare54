pub use crate::assets::{GameAssets, Lasers, Skeletons};
pub use crate::bullets::Bullet;
pub use crate::game_state::GameState;
pub use crate::indicators::DistantIndicator;
pub use crate::physics::{Collision, InertiaVolume};
pub use crate::player::Player;
pub use crate::trade_routes::{CargoSection, CargoShip};
use bevy::ecs::system::Command;
pub use bevy::prelude::*;
pub use bevy::utils::HashMap;
pub use bevy_spine::prelude::*;
pub use bevy_spine::rusty_spine::{c_interface::CTmpMut, Skeleton, Slot};
pub use bevy_spine::{SkeletonController, SpineBundle};

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
