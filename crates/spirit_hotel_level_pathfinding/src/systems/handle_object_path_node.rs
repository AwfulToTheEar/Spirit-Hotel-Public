use spirit_hotel_core::avian::prelude::Collider;
use bevy::prelude::*;
use spirit_hotel_level::tiled::prelude::{ObjectCreated, TiledEvent};
use spirit_hotel_pathfinding::components::prelude::PathNode;

pub fn handle_object_path_node(
    mut commands: Commands,
    mut e_object_created: MessageReader<TiledEvent<ObjectCreated>>,
    q_path_nodes: Query<&Children, With<PathNode>>,
    q_colliders: Query<Entity, (With<Collider>, With<ChildOf>)>,
) {
    for e in e_object_created.read() {
        let Ok(children) = q_path_nodes.get(e.origin) else {
            continue;
        };

        for entity in children.iter() {
            if !q_colliders.contains(entity) {
                continue;
            }

            commands.entity(entity).despawn();
        }
    }
}
