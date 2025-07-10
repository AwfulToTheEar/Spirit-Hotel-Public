use bevy::prelude::*;
use bevy_ecs_tiled::{map::TiledMapHandle, prelude::{TiledColliderMarker, TiledMap, TiledMapObject}};

use crate::game::level::components::prelude::InsideMap;

pub fn mark_objects_inside_map(
    mut commands: Commands,
    tiled_maps: Res<Assets<TiledMap>>,
    mut q_tiled_map_objects: Query<
        (Entity, &GlobalTransform, Option<&mut InsideMap>),
        (
            Or<(With<TiledMapObject>, With<TiledColliderMarker>)>,
            Changed<GlobalTransform>,
        ),
    >,
    q_maps: Query<(Entity, &TiledMapHandle, &GlobalTransform)>,
) {
    for (map_entity, handle, map_transform) in q_maps.iter() {
        let Some(tiled_map) = tiled_maps.get(handle.0.id()) else {
            continue;
        };

        let map_translation = map_transform.translation();
        let map_size = Vec2::new(tiled_map.rect.max.x, tiled_map.rect.max.y);

        for (entity, transform, inside_map_option) in q_tiled_map_objects.iter_mut() {
            let translation = transform.translation();

            if !(translation.x >= map_translation.x
                && translation.x < map_translation.x + map_size.x
                && translation.y <= map_translation.y
                && translation.y > map_translation.y - map_size.y)
            {
                continue;
            }

            let Some(mut inside_map) = inside_map_option else {
                commands.entity(entity).insert(InsideMap(map_entity));
                continue;
            };

            if inside_map.0 == map_entity {
                continue;
            }

            inside_map.0 = map_entity
        }
    }
}
