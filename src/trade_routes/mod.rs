use crate::prelude::*;

pub const ARENA_SIZE: f32 = 1000.0;
pub const HYPERDRIVE_SPEED: f32 = 500.0;

mod cargo_ships;
mod system;
pub use cargo_ships::*;
pub use system::*;

pub struct TradeRoutesPlugin;

impl Plugin for TradeRoutesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), spawn_starting_system)
            .add_systems(
                Update,
                (
                    update_system_indicators.run_if(in_state(GameState::Playing)),
                    pick_hyperdrive_target.run_if(in_state(GameState::Playing)),
                    engage_hyperdrive_system.run_if(in_state(GameState::Playing)),
                    initialize_local_region.run_if(in_state(GameState::Hyperdrive)),
                    cargo_ship_jet_animation_system,
                    cargo_ship_defense_system.run_if(in_state(GameState::Playing)),
                    cargo_ship_escape_system,
                    cargo_ship_drop_system.run_if(in_state(GameState::Playing)),
                ),
            );
    }
}
