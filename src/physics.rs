use crate::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity);
    }
}

#[derive(Component, Debug)]
pub struct InertiaVolume {
    pub velocity: Vec2,
    pub rotation: f32,
    pub rotation_velocity: f32,
    pub mass: f32,
    pub radius: f32,
}

const COLLISION_CHECK_TICKS: usize = 10;
const COLLISION_SPEED_BUFFER_DISTANCE: f32 = 300.0;

impl InertiaVolume {
    pub fn new(mass: f32, radius: f32) -> Self {
        Self {
            velocity: Vec2::ZERO,
            rotation: 0.0,
            rotation_velocity: 0.0,
            mass,
            radius,
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vec2) {
        self.velocity += impulse / self.mass;
    }

    pub fn apply_force(&mut self, force: Vec2, dt: f32) {
        self.velocity += force / self.mass * dt;
    }

    pub fn apply_thrust_force(&mut self, force_length: f32, dt: f32) {
        let force = Vec2::new(self.rotation.cos(), self.rotation.sin()) * force_length;
        self.apply_force(force, dt);
    }

    pub fn apply_thrust_force_limited(&mut self, force_length: f32, limit: f32, dt: f32) {
        let thrust_vector = Vec2::new(self.rotation.cos(), self.rotation.sin());
        let current_forward_speed = self.velocity.dot(thrust_vector);
        if force_length > 0.0 && current_forward_speed >= limit {
            return;
        } else if force_length < 0.0 && current_forward_speed <= -limit {
            return;
        }
        self.apply_force(thrust_vector * force_length, dt);
    }

    pub fn apply_thrust_braking(&mut self, braking: f32, dt: f32) -> f32 {
        let thrust_vector = Vec2::new(self.rotation.cos(), self.rotation.sin());
        let tangent_vector = self.velocity - self.velocity.project_onto(thrust_vector);
        if tangent_vector.length_squared() == 0.0 {
            return 0.0;
        }
        let right = tangent_vector.angle_between(thrust_vector) < 0.0;
        let braking_force = tangent_vector.normalize() * braking;
        self.apply_force(-braking_force, dt);
        tangent_vector.length() * if right { -1.0 } else { 1.0 }
    }

    pub fn apply_rotation_force(&mut self, rotation: f32, dt: f32) {
        self.rotation += rotation * dt;
    }

    pub fn find_collision(&self, other: &InertiaVolume, mut other_relative: Vec2) -> Option<usize> {
        let mut distance_squared = other_relative.length_squared();
        let radius_sum = self.radius + other.radius;
        let buffered_distance = radius_sum + COLLISION_SPEED_BUFFER_DISTANCE;
        if distance_squared > buffered_distance * buffered_distance {
            return None;
        }
        let mut ticks_left = COLLISION_CHECK_TICKS;
        let my_tick = self.velocity.normalize_or_zero();
        let other_tick = other.velocity.normalize_or_zero();
        while ticks_left > 0 && distance_squared > radius_sum * radius_sum {
            other_relative -= my_tick;
            other_relative += other_tick;
            distance_squared = other_relative.length_squared();
            ticks_left -= 1;
        }
        if distance_squared > radius_sum * radius_sum {
            return None;
        }
        Some(COLLISION_CHECK_TICKS - ticks_left)
    }
}

fn apply_velocity(time: Res<Time>, mut inertia_volumes: Query<(&mut Transform, &InertiaVolume)>) {
    for (mut transform, inertia_volume) in inertia_volumes.iter_mut() {
        transform.translation += inertia_volume.velocity.extend(0.0) * time.delta_seconds();
    }
}

#[cfg(test)]
mod physics_tests {
    use super::*;

    #[test]
    fn find_collision_easy() {
        let inertia_volume = InertiaVolume::new(1.0, 1.0);
        let other = InertiaVolume::new(1.0, 1.0);
        let diff = Vec2::new(1.0, 0.0);
        let collision = inertia_volume.find_collision(&other, diff);
        assert_eq!(collision, Some(0));
    }

    #[test]
    fn find_collision_tangent() {
        let mut going_right = InertiaVolume::new(1.0, 1.0);
        going_right.velocity = Vec2::new(1.0, 0.0);
        let mut going_down = InertiaVolume::new(1.0, 1.0);
        going_down.velocity = Vec2::new(0.0, 1.0);
        let diff = Vec2::new(5.0, -5.0);
        let collision = going_right.find_collision(&going_down, diff);
        assert_eq!(collision, Some(4));
    }

    #[test]
    fn find_collision_miss() {
        let mut going_right = InertiaVolume::new(1.0, 1.0);
        going_right.velocity = Vec2::new(1.0, 0.0);
        let mut going_down = InertiaVolume::new(1.0, 1.0);
        going_down.velocity = Vec2::new(0.0, -1.0);
        let diff = Vec2::new(5.0, -5.0);
        let collision = going_right.find_collision(&going_down, diff);
        assert_eq!(collision, None);
    }
}
