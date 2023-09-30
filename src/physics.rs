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
    pub mass: f32,
    pub radius: f32,
}

impl InertiaVolume {
    pub fn new(velocity: Vec2, rotation: f32, mass: f32, radius: f32) -> Self {
        Self {
            velocity,
            rotation,
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
}

fn apply_velocity(mut inertia_volumes: Query<(&mut Transform, &InertiaVolume)>) {
    for (mut transform, inertia_volume) in inertia_volumes.iter_mut() {
        transform.translation += inertia_volume.velocity.extend(0.0);
    }
}
