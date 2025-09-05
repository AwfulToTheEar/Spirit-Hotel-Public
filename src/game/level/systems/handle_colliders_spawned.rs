use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{ColliderCreated, TiledEvent, TiledLayer, TiledMap, TiledObject};

use crate::game::level::components::prelude::TiledCollidersSpawned;

pub fn handle_colliders_spawned(
    mut commands: Commands,
    e_collider_created: EventReader<TiledEvent<ColliderCreated>>,
    q_tiled_entities: Query<
        Entity,
        (
            Without<TiledCollidersSpawned>,
            Or<(With<TiledMap>, With<TiledLayer>, With<TiledObject>)>,
        ),
    >,
) {
    if e_collider_created.is_empty() {
        return;
    }

    for entity in q_tiled_entities {
        commands.entity(entity).insert(TiledCollidersSpawned);
    }
}
