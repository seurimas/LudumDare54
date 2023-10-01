use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt},
};
use bevy_spine::{Atlas, SkeletonData, SkeletonJson};

use crate::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_systems(
            OnExit(GameState::Loading),
            (create_lasers, create_skeletons),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "sprites/player.png")]
    pub player: Handle<Image>,
    #[asset(path = "sprites/jammer.png")]
    pub jammer: Handle<Image>,
    #[asset(path = "sprites/cargo_ship.png")]
    pub cargo_ship: Handle<Image>,
    #[asset(path = "sprites/indicator.png")]
    pub indicator: Handle<Image>,
    #[asset(path = "sprites/local_indicator.png")]
    pub local_indicator: Handle<Image>,
    #[asset(path = "sprites/exotic.png")]
    pub exotic: Handle<Image>,
    #[asset(path = "sprites/salvage.png")]
    pub salvage: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 8, rows = 1))]
    #[asset(path = "sprites/Upgrades.png")]
    pub upgrades: Handle<TextureAtlas>,
    // Need to load atlas and jsons, then create skeletons.
    #[asset(path = "spines/player_ship.atlas")]
    pub player_ship_atlas: Handle<Atlas>,
    #[asset(path = "spines/player_ship.json")]
    pub player_ship_json: Handle<SkeletonJson>,
    #[asset(path = "spines/cargo_ship.atlas")]
    pub cargo_ship_atlas: Handle<Atlas>,
    #[asset(path = "spines/cargo_ship.json")]
    pub cargo_ship_json: Handle<SkeletonJson>,
    // Music!
    #[asset(path = "music/title_theme.ogg")]
    pub title_theme: Handle<AudioSource>,
    #[asset(path = "music/engagement.ogg")]
    pub engagement: Handle<AudioSource>,
    #[asset(path = "music/home_theme.ogg")]
    pub home_theme: Handle<AudioSource>,
    #[asset(path = "music/space_theme.ogg")]
    pub space_theme: Handle<AudioSource>,
    #[asset(path = "music/game_over.ogg")]
    pub game_over: Handle<AudioSource>,
    #[asset(path = "music/retire.ogg")]
    pub retire: Handle<AudioSource>,
    // Sounds!
    #[asset(path = "sounds/cargo_ship_section_hit.ogg")]
    pub cargo_ship_section_hit: Handle<AudioSource>,
    #[asset(path = "sounds/cargo_ship_section_destroyed.ogg")]
    pub cargo_ship_section_destroyed: Handle<AudioSource>,
    #[asset(path = "sounds/cargo_ship_hyperdrive.ogg")]
    pub cargo_ship_hyperdrive: Handle<AudioSource>,
    #[asset(path = "sounds/cargo_ship_laser.ogg")]
    pub cargo_ship_laser: Handle<AudioSource>,
    #[asset(path = "sounds/pickup.ogg")]
    pub pickup: Handle<AudioSource>,
    #[asset(path = "sounds/pickup_xm.ogg")]
    pub pickup_xm: Handle<AudioSource>,
    #[asset(path = "sounds/player_shield_hit.ogg")]
    pub player_shield_hit: Handle<AudioSource>,
    #[asset(path = "sounds/player_hull_hit.ogg")]
    pub player_hull_hit: Handle<AudioSource>,
    #[asset(path = "sounds/player_destroyed.ogg")]
    pub player_destroyed: Handle<AudioSource>,
    #[asset(path = "sounds/player_laser.ogg")]
    pub player_laser: Handle<AudioSource>,
    #[asset(path = "sounds/player_hyperdrive.ogg")]
    pub player_hyperdrive: Handle<AudioSource>,
    #[asset(path = "sounds/player_jammed.ogg")]
    pub player_jammed: Handle<AudioSource>,
    #[asset(path = "sounds/upgrade.ogg")]
    pub upgrade: Handle<AudioSource>,
    #[asset(path = "sounds/fail.ogg")]
    pub fail: Handle<AudioSource>,
    #[asset(path = "sounds/deploy_jammer.ogg")]
    pub deploy_jammer: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct Skeletons {
    pub player_ship: Handle<SkeletonData>,
    pub cargo_ship: Handle<SkeletonData>,
}

#[derive(Resource)]
pub struct Lasers {
    pub player_laser_mesh: Handle<Mesh>,
    pub player_laser_material: Handle<ColorMaterial>,
    // Cargo lasers!
    pub cargo_ship_laser_mesh: Handle<Mesh>,
    pub cargo_ship_laser_material: Handle<ColorMaterial>,
    // Jammer pixels (not lasers, oh well)!
    pub jammer_mesh: Handle<Mesh>,
    pub jammer_material: Handle<ColorMaterial>,
    // Star pixels (not lasers, oh well)!
    pub star_mesh: Handle<Mesh>,
    pub star_material: Handle<ColorMaterial>,
}

fn create_skeletons(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut skeletons: ResMut<Assets<SkeletonData>>,
) {
    let player_ship_skeleton = SkeletonData::new_from_json(
        assets.player_ship_json.clone(),
        assets.player_ship_atlas.clone(),
    );
    let player_ship = skeletons.add(player_ship_skeleton);

    let cargo_ship_skeleton = SkeletonData::new_from_json(
        assets.cargo_ship_json.clone(),
        assets.cargo_ship_atlas.clone(),
    );
    let cargo_ship = skeletons.add(cargo_ship_skeleton);

    commands.insert_resource(Skeletons {
        player_ship,
        cargo_ship,
    });
}

fn create_lasers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_laser_mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(5., 2.5))));
    let player_laser_material =
        materials.add(ColorMaterial::from(Color::rgba(7.5, 0.0, 7.5, 10.0)));

    let cargo_ship_laser_mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(7.5, 3.75))));
    let cargo_ship_laser_material =
        materials.add(ColorMaterial::from(Color::rgba(7.5, 7.5, 0.0, 15.0)));

    let jammer_mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(2., 2.))));
    let jammer_material = materials.add(ColorMaterial::from(Color::rgba(3.0, 3.0, 0.0, 1.0)));

    let star_mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1., 1.))));
    let star_material = materials.add(ColorMaterial::from(Color::rgba(3.0, 3.0, 3.0, 1.0)));

    commands.insert_resource(Lasers {
        player_laser_mesh,
        player_laser_material,
        cargo_ship_laser_mesh,
        cargo_ship_laser_material,
        jammer_mesh,
        jammer_material,
        star_mesh,
        star_material,
    });
}
