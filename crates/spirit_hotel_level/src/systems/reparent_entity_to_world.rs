use crate::components::prelude::ReparentToWorld;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledWorld;

pub fn reparent_entity_to_world(
    e_reparent_to_world_added: On<Add, ReparentToWorld>,
    mut commands: Commands,
    s_world: Single<Entity, With<TiledWorld>>,
) {
    let world = s_world.into_inner();
    commands
        .entity(e_reparent_to_world_added.entity)
        .set_parent_in_place(world)
        .remove::<ReparentToWorld>();
}
