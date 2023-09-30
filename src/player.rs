use crate::prelude::*;

// Define a plugin for the player.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add the player to the world.
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    player_camera_system,
                    player_movement_system,
                    player_animate_system,
                ),
            );
    }
}

// Define a component for the player.
#[derive(Component)]
pub struct Player {
    pub thrust: f32,
    pub side_braking: f32,
    pub speed_limit: f32,
    pub engine_strength: f32,
    pub thrust_braking_strength: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            thrust: 0.0,
            side_braking: 0.0,
            speed_limit: 3.0,
            engine_strength: 5.0,
            thrust_braking_strength: 2.0,
        }
    }
}

// Define a system to spawn the player.
fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Spawn a sprite for the player.
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: game_assets.player.clone(),
            ..Default::default()
        },
        InertiaVolume::new(Vec2::new(0.0, 0.0), 0.0, 1.0, 1.0),
        Player::new(),
    ));
    commands.spawn((Camera2dBundle::default(),));
}

fn player_camera_system(
    mut queries: ParamSet<(
        Query<(&Camera2d, &mut Transform)>,
        Query<(&Player, &Transform)>,
    )>,
) {
    let mut center_transform = Transform::from_xyz(0.0, 0.0, 10.0);
    for (_player, player_transform) in queries.p1().iter() {
        center_transform.translation = player_transform.translation;
    }
    for (_camera, mut transform) in queries.p0().iter_mut() {
        transform.translation = center_transform.translation;
    }
}

fn player_animate_system(mut players: Query<(&Player, &InertiaVolume, &mut Transform)>) {
    for (_player, inertia, mut transform) in players.iter_mut() {
        transform.rotation = Quat::from_rotation_z(inertia.rotation);
    }
}

fn player_movement_system(
    time: Res<Time>,
    mut players: Query<(&mut Player, &mut InertiaVolume)>,
    input: Res<Input<KeyCode>>,
) {
    let dt = time.delta_seconds();
    for (mut player, mut inertia) in players.iter_mut() {
        if input.pressed(KeyCode::W) {
            inertia.apply_thrust_force_limited(player.engine_strength, player.speed_limit, dt);
            player.thrust = 1.0;
            player.side_braking = inertia.apply_thrust_braking(player.thrust_braking_strength, dt);
            println!(
                "thrust: {} side_braking: {}",
                player.thrust, player.side_braking
            );
        } else if input.pressed(KeyCode::S) {
            inertia.apply_thrust_force_limited(-player.engine_strength, player.speed_limit, dt);
            player.thrust = -1.0;
            player.side_braking = inertia.apply_thrust_braking(player.thrust_braking_strength, dt);
            println!(
                "thrust: {} side_braking: {}",
                player.thrust, player.side_braking
            );
        } else {
            player.thrust = 0.0;
            player.side_braking = 0.0;
        }
        if input.pressed(KeyCode::A) {
            inertia.apply_rotation_force(5.0, dt);
        }
        if input.pressed(KeyCode::D) {
            inertia.apply_rotation_force(-5.0, dt);
        }
    }
}
