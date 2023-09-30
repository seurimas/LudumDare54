use crate::prelude::*;

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_debug_drops);
        app.add_systems(
            Update,
            (
                player_pickup_system,
                cargo_ship_drop_system.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

#[derive(Component)]
pub enum Pickup {
    ExoticMaterial(f32),
    Salvage(f32),
}

fn spawn_debug_drops(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(300.0, 0.0, 0.0),
            texture: game_assets.exotic.clone(),
            sprite: Sprite {
                color: Color::rgba(10., 10., 0., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        InertiaVolume::new(1.0, 16.0),
        Pickup::ExoticMaterial(100.0),
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

fn cargo_ship_drop_system(mut commands: Commands, game_assets: Res<GameAssets>) {
    // TODO
}
