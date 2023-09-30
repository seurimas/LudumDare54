use crate::prelude::*;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (cargo_ship_damage_system,));
    }
}

#[derive(Component, PartialEq, Eq)]
pub enum Bullet {
    Player,
    Enemy,
}

fn cargo_ship_damage_system(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    player_bullets: Query<(Entity, &Bullet)>,
    cargo_sections: Query<(&Parent, &CargoSection)>,
    mut cargo_ship: Query<(&mut CargoShip, &mut Spine)>,
) {
    for collision in collisions.iter() {
        if let Ok((bullet_entity, bullet)) = player_bullets.get(collision.e0) {
            if let Ok((cargo_ship_ref, cargo_section)) = cargo_sections.get(collision.e1) {
                if bullet == &Bullet::Player {
                    if let Ok((mut cargo_ship, mut ship_skeleton)) =
                        cargo_ship.get_mut(**cargo_ship_ref)
                    {
                        unsafe {
                            ship_skeleton
                                .animation_state
                                .set_animation_by_name_unchecked(
                                    cargo_section.index,
                                    cargo_section.hit_animation,
                                    false,
                                );
                        }
                        if let Some(mut bullet_entity) = commands.get_entity(bullet_entity) {
                            bullet_entity.despawn();
                        }
                        cargo_ship.damage_section(cargo_section.index, 10.0);
                    }
                }
            }
        }
    }
}
