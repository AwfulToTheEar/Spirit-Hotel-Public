use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledColliderCreated, TiledMapLayer, TiledMapMarker, TiledMapObject};

use crate::game::level::components::prelude::TiledCollidersSpawned;

//this breaks if there are no objects with collision or no layers with collision in the world
pub fn handle_colliders_spawned(
    mut commands: Commands,
    e_collider_created: EventReader<TiledColliderCreated>,
    q_tiled_entities: Query<
        Entity,
        (
            Without<TiledCollidersSpawned>,
            Or<(
                With<TiledMapMarker>,
                With<TiledMapLayer>,
                With<TiledMapObject>,
            )>,
        ),
    >,
    mut counter: Local<i16>,
) {
    if e_collider_created.is_empty() {
        return;
    }

    *counter += 1;

    if *counter < 2 {
        return;
    }

    *counter = 0;

    for entity in q_tiled_entities {
        commands.entity(entity).insert(TiledCollidersSpawned);
    }
}
