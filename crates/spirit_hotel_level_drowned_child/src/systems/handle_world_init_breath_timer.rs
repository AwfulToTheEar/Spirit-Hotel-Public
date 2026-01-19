use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{MapCreated, TiledEvent, TiledWorld};
use spirit_hotel_drowned_child::components::prelude::{BreathTimer, InitBreathTimer};

pub fn handle_world_init_breath_timer(
    e_map_created: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    q_init_breath_timer: Query<&InitBreathTimer>,
    s_tiled_world: Single<Entity, With<TiledWorld>>,
) {

    let Ok(init_breath_timer) = q_init_breath_timer.get(e_map_created.origin) else {
        return;
    };

    let entity = s_tiled_world.into_inner();

    commands
        .entity(entity)
        .insert(BreathTimer(Timer::from_seconds(
            init_breath_timer.0 as f32,
            TimerMode::Once,
        )));
    commands.entity(e_map_created.origin).remove::<InitBreathTimer>();

}
