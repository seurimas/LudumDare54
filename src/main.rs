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
use bevy_spine::SpinePlugin;
use bullets::BulletsPlugin;
use game_over::GameOverPlugin;
use home::HomePlugin;
use indicators::IndicatorsPlugin;
use intro::IntroPlugin;
use jamming::JammingPlugin;
use physics::PhysicsPlugin;
use pickups::PickupsPlugin;
use player::PlayerPlugin;
use space_pixels::SpacePixelsPlugin;
use trade_routes::TradeRoutesPlugin;
use ui::GameUiPlugin;

use crate::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            SpinePlugin,
            GameOverPlugin,
            IntroPlugin,
            JammingPlugin,
            BulletsPlugin,
            TradeRoutesPlugin,
            IndicatorsPlugin,
            GameAssetsPlugin,
            PhysicsPlugin,
            PickupsPlugin,
            PlayerPlugin,
            GameUiPlugin,
            SpacePixelsPlugin,
            HomePlugin,
        ))
        .run();
}
