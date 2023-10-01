use crate::prelude::*;

pub struct SpacePixelsPlugin;

impl Plugin for SpacePixelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_space_pixels);
    }
}

const JAMMER_VELOCITY_SCALE: f32 = 10.0;
const JAMMER_ACCELERATION_SCALE: f32 = 20.0;

#[derive(Component)]
pub struct SpacePixel {
    pub lifetime: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl SpacePixel {
    pub fn random_jammer() -> Self {
        let mut rng = rand::thread_rng();
        let lifetime = rng.gen_range(0.0..1.0);
        let vel_range = -JAMMER_VELOCITY_SCALE..JAMMER_VELOCITY_SCALE;
        let velocity = Vec2::new(
            rng.gen_range(vel_range.clone()),
            rng.gen_range(vel_range.clone()),
        );
        let acc_range = -JAMMER_ACCELERATION_SCALE..JAMMER_ACCELERATION_SCALE;
        let acceleration = Vec2::new(
            rng.gen_range(acc_range.clone()),
            rng.gen_range(acc_range.clone()),
        );
        Self {
            lifetime,
            velocity,
            acceleration,
        }
    }
}

pub fn update_space_pixels(
    time: Res<Time>,
    mut commands: Commands,
    mut jammer_pixels: Query<(Entity, &mut SpacePixel, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut jammer_pixel, mut transform) in jammer_pixels.iter_mut() {
        jammer_pixel.lifetime -= dt;
        if jammer_pixel.lifetime < 0.0 {
            commands.entity(entity).despawn();
        } else {
            let delta_v = jammer_pixel.acceleration * dt;
            jammer_pixel.velocity += delta_v;
            transform.translation += jammer_pixel.velocity.extend(0.0) * dt;
        }
    }
}
