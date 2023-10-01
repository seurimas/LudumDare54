use crate::prelude::*;

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_debug_drops);
        app.add_systems(Update, (player_pickup_system,));
    }
}

#[derive(Component)]
pub enum Pickup {
    ExoticMaterial(f32),
    Salvage(f32),
}

fn spawn_debug_drops(mut commands: Commands, game_assets: Res<GameAssets>) {
    spawn_exotic(300., 0., &mut commands, game_assets.exotic.clone(), 100.);
}

pub fn spawn_exotic(
    x: f32,
    y: f32,
    mut commands: &mut Commands<'_, '_>,
    texture: Handle<Image>,
    value: f32,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture,
            sprite: Sprite {
                color: Color::rgba(10., 10., 0., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        InertiaVolume::new(1.0, 16.0),
        Pickup::ExoticMaterial(value),
        Regional,
        Jammer {
            radius: rand::thread_rng().gen_range(1000.0..1500.0),
            progress: 0.0,
        },
    ));
}

pub fn spawn_salvage(
    x: f32,
    y: f32,
    velocity: Vec2,
    mut commands: &mut Commands<'_, '_>,
    texture: Handle<Image>,
    value: f32,
) {
    let mut inertia_volume = InertiaVolume::new(1.0, 8.0);
    inertia_volume.velocity = velocity;
    inertia_volume.rotation_velocity = 0.1;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture,
            sprite: Sprite {
                color: Color::rgba(2., 2., 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        Regional,
        inertia_volume,
        Pickup::Salvage(value),
    ));
}

fn player_pickup_system(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    mut players: Query<&mut Player>,
    pickups: Query<&Pickup>,
) {
    for collision in collisions.iter() {
        if let Ok(mut player) = players.get_mut(collision.e0) {
            if let Ok(pickup) = pickups.get(collision.e1) {
                match pickup {
                    Pickup::ExoticMaterial(amount) => {
                        player.exotic_material += amount;
                    }
                    Pickup::Salvage(amount) => {
                        player.salvage += amount;
                    }
                }
                if let Some(mut pickup_entity) = commands.get_entity(collision.e1) {
                    pickup_entity.despawn();
                }
            }
        }
    }
}
