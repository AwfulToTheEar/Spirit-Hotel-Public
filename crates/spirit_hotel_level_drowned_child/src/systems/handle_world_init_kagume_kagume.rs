use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMap, TiledWorld};
use spirit_hotel_drowned_child::components::prelude::{InitKagumeKagume, KagumeKagume};

pub fn handle_world_init_kagume_kagume(
    mut commands: Commands,
    s_init_kagume_kagume: Single<Entity, (Added<InitKagumeKagume>, With<TiledMap>)>,
    s_tiled_world: Single<Entity, With<TiledWorld>>,
) {
    let map_entity = s_init_kagume_kagume.into_inner();

    let world_entity = s_tiled_world.into_inner();

    commands
        .entity(world_entity)
        .insert(KagumeKagume::default());
    commands.entity(map_entity).remove::<InitKagumeKagume>();
}
