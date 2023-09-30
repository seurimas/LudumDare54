use crate::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Collision>()
            .init_resource::<SpacialGrid>()
            .add_systems(
                Update,
                (maintain_spacial_grid, generate_collisions, apply_velocity),
            );
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

const COLLISION_TICK_LENGTH: f32 = 1.0 / 1000.0;

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
        self.apply_offset_thrust_force_limited(force_length, 0., limit, dt);
    }

    pub fn apply_offset_thrust_force_limited(
        &mut self,
        force_length: f32,
        rotation_offset: f32,
        limit: f32,
        dt: f32,
    ) {
        let rotation = self.rotation + rotation_offset;
        let thrust_vector = Vec2::new(rotation.cos(), rotation.sin());
        let current_forward_speed = self.velocity.dot(thrust_vector);
        if force_length > 0.0 && current_forward_speed >= limit {
            return;
        } else if force_length < 0.0 && current_forward_speed <= -limit {
            return;
        }
        self.apply_force(thrust_vector * force_length, dt);
    }

    pub fn apply_thrust_braking(&mut self, braking: f32, dt: f32) -> f32 {
        self.apply_offset_thrust_braking(braking, 0.0, dt)
    }

    pub fn apply_offset_thrust_braking(
        &mut self,
        braking: f32,
        rotation_offset: f32,
        dt: f32,
    ) -> f32 {
        let rotation = self.rotation + rotation_offset;
        let thrust_vector = Vec2::new(rotation.cos(), rotation.sin());
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

    pub fn find_collision(
        &self,
        other: &InertiaVolume,
        mut other_relative: Vec2,
        dt: f32,
    ) -> Option<Vec2> {
        let mut distance_squared = other_relative.length_squared();
        let radius_sum = self.radius + other.radius;
        let ticks = (dt / COLLISION_TICK_LENGTH) as usize + 1;
        let tick_length = dt / ticks as f32;
        let my_tick = self.velocity * tick_length;
        let other_tick = other.velocity * tick_length;
        let mut ticks_left = ticks;
        while ticks_left > 0 && distance_squared > radius_sum * radius_sum {
            other_relative -= my_tick;
            other_relative += other_tick;
            distance_squared = other_relative.length_squared();
            ticks_left -= 1;
        }
        if distance_squared > radius_sum * radius_sum {
            return None;
        }
        Some((ticks - ticks_left) as f32 * my_tick)
    }
}

#[derive(Resource, Debug)]
pub struct SpacialGrid {
    grid: HashMap<(i32, i32), Vec<Entity>>,
    cell_size: f32,
}

struct SpaceIterator<'a> {
    space: &'a SpacialGrid,
    index: usize,
    x: i32,
    y: i32,
    cell_index: usize,
}

impl<'a> SpaceIterator<'a> {
    fn new(space: &'a SpacialGrid, cell: (i32, i32)) -> Self {
        Self {
            space,
            index: 0,
            x: cell.0,
            y: cell.1,
            cell_index: 0,
        }
    }
}

impl<'a> Iterator for SpaceIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cell_index >= 9 {
            return None;
        }
        let x = self.x - 1 + self.cell_index as i32 % 3;
        let y = self.y - 1 + self.cell_index as i32 / 3;
        let cell = self.space.grid.get(&(x, y));
        if let Some(cell) = cell {
            if self.index < cell.len() {
                let entity = cell[self.index];
                self.index += 1;
                return Some(entity);
            } else {
                self.index = 0;
                self.cell_index += 1;
                return self.next();
            }
        } else {
            self.index = 0;
            self.cell_index += 1;
            return self.next();
        }
    }
}

impl SpacialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            grid: HashMap::new(),
            cell_size,
        }
    }

    pub fn insert(&mut self, entity: Entity, position: Vec2) {
        let cell = self.cell(position);
        self.grid.entry(cell).or_default().push(entity);
    }

    pub fn remove(&mut self, entity: Entity, position: Vec2) {
        let cell = self.cell(position);
        if let Some(entities) = self.grid.get_mut(&cell) {
            entities.retain(|e| *e != entity);
        }
    }

    pub fn update(&mut self, entity: Entity, old_position: Vec2, new_position: Vec2) {
        if self.cell(old_position) == self.cell(new_position) {
            return;
        }
        self.remove(entity, old_position);
        self.insert(entity, new_position);
    }

    fn retain(&mut self, predicate: impl Fn(&Entity) -> bool) {
        for entities in self.grid.values_mut() {
            entities.retain(&predicate);
        }
        self.grid.retain(|_, entities| !entities.is_empty());
    }

    pub fn query<'a>(&'a self, position: Vec2) -> SpaceIterator<'a> {
        SpaceIterator::new(self, self.cell(position))
    }

    fn cell(&self, position: Vec2) -> (i32, i32) {
        (
            (position.x / self.cell_size).floor() as i32,
            (position.y / self.cell_size).floor() as i32,
        )
    }
}

impl Default for SpacialGrid {
    fn default() -> Self {
        Self::new(200.0)
    }
}

#[derive(Component, Debug)]
pub struct SpacialReference(Vec2);

#[derive(Event, Debug, Clone)]
pub struct Collision {
    pub e0: Entity,
    pub e1: Entity,
    pub location: Vec2,
}

fn maintain_spacial_grid(
    mut commands: Commands,
    mut spacial_grid: ResMut<SpacialGrid>,
    mut inertia_volumes: Query<
        (Entity, &GlobalTransform, Option<&mut SpacialReference>),
        With<InertiaVolume>,
    >,
) {
    spacial_grid.retain(|e| inertia_volumes.get(*e).is_ok());
    for (entity, transform, previous) in inertia_volumes.iter_mut() {
        let position = transform.translation().truncate();
        if let Some(mut previous) = previous {
            if previous.0 == position {
                continue;
            } else {
                spacial_grid.update(entity, previous.0, position);
                previous.0 = position;
            }
        } else {
            spacial_grid.insert(entity, position);
            commands.entity(entity).insert(SpacialReference(position));
        }
    }
}

fn generate_collisions(
    time: Res<Time>,
    spacial_grid: Res<SpacialGrid>,
    inertia_volumes: Query<(Entity, &GlobalTransform, &InertiaVolume)>,
    mut collisions: EventWriter<Collision>,
) {
    let dt = time.delta_seconds();
    for (entity, transform, inertia_volume) in inertia_volumes.iter() {
        let position = transform.translation().truncate();
        for other in spacial_grid.query(position) {
            if other == entity {
                continue;
            }
            let (_, other_transform, other_volume) = inertia_volumes.get(other).unwrap();
            let other_position = other_transform.translation().truncate();
            let diff = position - other_position;
            if let Some(collision) = inertia_volume.find_collision(other_volume, diff, dt) {
                println!("collision: {}", collision);
                collisions.send(Collision {
                    e0: entity,
                    e1: other,
                    location: position + collision,
                });
            }
        }
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
        let collision = inertia_volume.find_collision(&other, diff, 1.0 / 60.0);
        assert_eq!(collision, Some(Vec2::new(0.0, 0.0)));
    }

    #[test]
    fn find_collision_tangent() {
        let mut going_right = InertiaVolume::new(1.0, 1.0);
        going_right.velocity = Vec2::new(500.0, 0.0);
        let mut going_down = InertiaVolume::new(1.0, 1.0);
        going_down.velocity = Vec2::new(0.0, 500.0);
        let diff = Vec2::new(10.0, -10.0);
        let collision = going_right.find_collision(&going_down, diff, 1.0 / 30.0);
        assert_eq!(collision, Some(Vec2::new(0.01764706 * 500., 0.)));
    }

    #[test]
    fn find_collision_miss() {
        let mut going_right = InertiaVolume::new(1.0, 1.0);
        going_right.velocity = Vec2::new(1.0, 0.0);
        let mut going_down = InertiaVolume::new(1.0, 1.0);
        going_down.velocity = Vec2::new(0.0, -1.0);
        let diff = Vec2::new(5.0, -5.0);
        let collision = going_right.find_collision(&going_down, diff, 1.0 / 60.0);
        assert_eq!(collision, None);
    }
}
