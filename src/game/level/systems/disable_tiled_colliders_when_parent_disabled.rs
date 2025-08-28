use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use bevy_ecs_tiled::prelude::{TiledColliderOrigin};

use crate::game::level::components::prelude::DisabledByParent;

pub fn disable_tiled_colliders_when_parent_disabled(
    mut commands: Commands,
    q_colliders: Query<(Entity, &ChildOf), With<TiledColliderOrigin>>,
    q_parents: Query<Entity, With<Disabled>>,
) {
    for (entity, child_of) in q_colliders.iter() {
        if !q_parents.contains(child_of.0) {
            continue;
        };

        commands.entity(entity).insert((DisabledByParent, Disabled));
    }
}
