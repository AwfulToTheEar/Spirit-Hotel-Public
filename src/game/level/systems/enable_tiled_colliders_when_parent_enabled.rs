use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use bevy_ecs_tiled::prelude::TiledColliderMarker;

use crate::game::level::components::prelude::DisabledByParent;

pub fn enable_tiled_colliders_when_parent_enabled(
    mut commands: Commands,
    q_colliders: Query<
        (Entity, &ChildOf),
        (
            With<DisabledByParent>,
            With<Disabled>,
            With<TiledColliderMarker>,
        ),
    >,
    q_parents: Query<Entity>,
) {
    for (entity, child_of) in q_colliders.iter() {
        if !q_parents.contains(child_of.0) {
            continue;
        };

        commands
            .entity(entity)
            .remove::<(DisabledByParent, Disabled)>();
    }
}
