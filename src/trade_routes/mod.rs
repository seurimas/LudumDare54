use crate::{
    game_state,
    home::{Career, HomeInSystem},
    player,
    prelude::*,
};

pub const ARENA_SIZE: f32 = 1000.0;
pub const HYPERDRIVE_SPEED: f32 = 500.0;

mod cargo_ships;
mod system;
pub use cargo_ships::*;
pub use system::*;

pub struct TradeRoutesPlugin;

impl Plugin for TradeRoutesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                maintain_current_system,
                handle_music.run_if(not(in_state(GameState::Loading))),
                update_system_indicators.run_if(in_state(GameState::Playing)),
                pick_hyperdrive_target.run_if(in_state(GameState::Playing)),
                engage_hyperdrive_system.run_if(in_state(GameState::Playing)),
                initialize_local_region.run_if(in_state(GameState::Hyperdrive)),
                cargo_ship_jet_animation_system,
                cargo_ship_defense_system.run_if(in_state(GameState::Playing)),
                cargo_ship_escape_system.run_if(in_state(GameState::Playing)),
                cargo_ship_drop_system.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

#[derive(Component)]
pub struct Music;

#[derive(PartialEq)]
enum ActiveSong {
    Title,
    Space,
    Hyperdrive,
    Engagement,
    Home,
    GameOver,
    Retire,
}

fn handle_music(
    game_assets: Res<GameAssets>,
    game_state: Res<State<GameState>>,
    mut commands: Commands,
    career: Res<Career>,
    query: Query<(Entity, &Handle<AudioSource>, &AudioSink), With<Music>>,
    cargo_ships: Query<&CargoShip>,
    home_system: Query<(&HomeInSystem, Option<&CurrentSystemRegion>)>,
    player: Query<&player::Player>,
) {
    let mut current_song = ActiveSong::Space;
    if cargo_ships.iter().any(|cargo_ship| cargo_ship.aggressed) {
        current_song = ActiveSong::Engagement;
    }
    if home_system
        .iter()
        .any(|(_, current_system_region)| current_system_region.is_some())
    {
        current_song = ActiveSong::Home;
    }
    if *game_state == GameState::Hyperdrive {
        current_song = ActiveSong::Hyperdrive;
    }
    if *game_state == GameState::GameOver {
        current_song = ActiveSong::GameOver;
    }
    if *game_state == GameState::Retire {
        current_song = ActiveSong::Retire;
    }
    if !career.intro_complete() {
        current_song = ActiveSong::Title;
    }
    let new_handle = match current_song {
        ActiveSong::Hyperdrive => None,
        ActiveSong::Space => Some(game_assets.space_theme.clone()),
        ActiveSong::Title => Some(game_assets.title_theme.clone()),
        ActiveSong::Engagement => Some(game_assets.engagement.clone()),
        ActiveSong::Home => Some(game_assets.home_theme.clone()),
        ActiveSong::GameOver => Some(game_assets.game_over.clone()),
        ActiveSong::Retire => Some(game_assets.retire.clone()),
    };
    let settings = if current_song != ActiveSong::GameOver && current_song != ActiveSong::Retire {
        PlaybackSettings::LOOP
    } else {
        PlaybackSettings::ONCE
    };
    if query.is_empty() {
        if new_handle.is_some() {
            commands.spawn((
                AudioBundle {
                    source: new_handle.unwrap(),
                    settings,
                },
                Music,
            ));
        }
    } else {
        let (music_entity, handle, audio_sink) = query.single();
        if let Some(new_handle) = new_handle {
            if new_handle != *handle {
                audio_sink.stop();
                commands.entity(music_entity).despawn();
                commands.spawn((
                    AudioBundle {
                        source: new_handle,
                        settings,
                    },
                    Music,
                ));
            }
        } else {
            commands.entity(music_entity).despawn();
        }
    }
}
