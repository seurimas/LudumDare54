use crate::prelude::*;

#[derive(Component)]
pub struct Jammable;

#[derive(Component)]
pub struct Jammed;

#[derive(Component)]
pub struct Jammer {
    pub radius: f32,
}

pub fn insert_jammed_around_jammer_system(
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(Entity, &Transform), With<Jammable>>,
        Query<(&Transform, &Jammer)>,
    )>,
) {
    let jammables = queries
        .p0()
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect::<Vec<_>>();
    for (jammer_transform, jammer) in queries.p1().iter() {
        for (jammed_entity, jammed_location) in &jammables {
            if jammer_transform.translation.distance(*jammed_location) < jammer.radius {
                commands.entity(*jammed_entity).insert(Jammed);
            } else {
                commands.entity(*jammed_entity).remove::<Jammed>();
            }
        }
    }
}

fn toggle_hyperdrive_enabled(mut slot: CTmpMut<Skeleton, Slot>, enabled: bool, pulse: f32) {
    if enabled {
        slot.color_mut().r = 1.0;
        slot.color_mut().g = 10.0;
        slot.color_mut().b = 1.0;
        slot.color_mut().a = 3.0 + 2.0 * pulse.sin();
    } else {
        slot.color_mut().r = 10.0;
        slot.color_mut().g = 1.0;
        slot.color_mut().b = 1.0;
        slot.color_mut().a = 1.0;
    }
}

pub fn indicate_jamming_on_skeleton(
    mut pulsing: Local<f32>,
    time: Res<Time>,
    mut query: Query<(&mut Spine, Option<&Jammed>), With<Jammable>>,
) {
    *pulsing += time.delta_seconds();
    for (mut spine, jammed) in query.iter_mut() {
        if jammed.is_some() {
            if let Some(mut left) = spine.skeleton.find_slot_mut("hyperdrive_left") {
                toggle_hyperdrive_enabled(left, false, *pulsing);
            }
            if let Some(mut right) = spine.skeleton.find_slot_mut("hyperdrive_right") {
                toggle_hyperdrive_enabled(right, false, *pulsing);
            }
        } else {
            if let Some(mut left) = spine.skeleton.find_slot_mut("hyperdrive_left") {
                toggle_hyperdrive_enabled(left, true, *pulsing);
            }
            if let Some(mut right) = spine.skeleton.find_slot_mut("hyperdrive_right") {
                toggle_hyperdrive_enabled(right, true, *pulsing);
            }
        }
    }
}
