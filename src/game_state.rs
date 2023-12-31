use crate::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Loading,
    Playing,
    Hyperdrive,
    Home,
    GameOver,
    Retire,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Loading
    }
}
