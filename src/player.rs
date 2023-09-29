use crate::prelude::*;

// Define a plugin for the player.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add the player to the world.
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (player_movement_system, player_animate_system));
    }
}

// Define a component for the player.
#[derive(Component)]
struct Player;

// Define a system to spawn the player.
fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Spawn a sprite for the player.
    let player = commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: game_assets.player.clone(),
            ..Default::default()
        },
        InertiaVolume::new(Vec2::new(0.0, 0.0), 0.0, 1.0, 1.0),
        Player,
    ));
    let camera = commands.spawn((Camera2dBundle::default(),));
}

fn player_animate_system(mut players: Query<(&Player, &InertiaVolume, &mut Transform)>) {
    for (_player, inertia, mut transform) in players.iter_mut() {
        transform.rotation = Quat::from_rotation_z(inertia.rotation);
    }
}

fn player_movement_system(
    time: Res<Time>,
    mut players: Query<(&Player, &mut InertiaVolume)>,
    input: Res<Input<KeyCode>>,
) {
    let dt = time.delta_seconds();
    for (_player, mut inertia) in players.iter_mut() {
        if input.pressed(KeyCode::W) {
            inertia.apply_thrust_force(5.0, dt);
        }
        if input.pressed(KeyCode::S) {
            inertia.apply_thrust_force(-5.0, dt);
        }
        if input.pressed(KeyCode::A) {
            inertia.apply_rotation_force(5.0, dt);
        }
        if input.pressed(KeyCode::D) {
            inertia.apply_rotation_force(-5.0, dt);
        }
    }
}
