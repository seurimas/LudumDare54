mod assets;
mod bullets;
mod game_state;
mod indicators;
mod physics;
mod pickups;
mod player;
mod prelude;
mod trade_routes;
mod turrets;

#[macro_use]
extern crate lazy_static;
use assets::GameAssetsPlugin;
use bevy_spine::SpinePlugin;
use bullets::BulletsPlugin;
use indicators::IndicatorsPlugin;
use physics::PhysicsPlugin;
use pickups::PickupsPlugin;
use player::PlayerPlugin;
use trade_routes::TradeRoutesPlugin;

use crate::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            SpinePlugin,
            BulletsPlugin,
            TradeRoutesPlugin,
            IndicatorsPlugin,
            GameAssetsPlugin,
            PhysicsPlugin,
            PickupsPlugin,
            PlayerPlugin,
        ))
        .run();
}
