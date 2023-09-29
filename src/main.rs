mod assets;
mod game_state;
mod physics;
mod player;
mod prelude;

#[macro_use]
extern crate lazy_static;
use assets::GameAssetsPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

use crate::prelude::*;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins((GameAssetsPlugin, PhysicsPlugin, PlayerPlugin))
        .run();
}
