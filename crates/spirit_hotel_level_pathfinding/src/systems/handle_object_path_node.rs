use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{ColliderCreated, TiledEvent};
use spirit_hotel_pathfinding::components::prelude::PathNode;

pub fn handle_object_path_node(
    e_object_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
    q_path_nodes: Query<Entity, With<PathNode>>,
) {
    if !q_path_nodes.contains(e_object_created.event.collider_of.0) {
        return;
    };

commands.entity(e_object_created.origin).despawn();
}
