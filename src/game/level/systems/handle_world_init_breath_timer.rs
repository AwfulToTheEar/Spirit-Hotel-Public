use crate::game::drowned_child::components::prelude::{BreathTimer, InitBreathTimer};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{MapCreated, TiledEvent, TiledWorld};

pub fn handle_world_init_breath_timer(
    mut commands: Commands,
    s_world: Single<Entity, With<TiledWorld>>,
    mut e_map_created: EventReader<TiledEvent<MapCreated>>,
    q_init_breath_timer: Query<(Entity, &InitBreathTimer)>,
) {
    let world = s_world.into_inner();

    for event in e_map_created.read() {
        let Ok((map, init_breath_timer)) = q_init_breath_timer.get(event.origin) else {
            continue;
        };

        commands
            .entity(world)
            .insert(BreathTimer(Timer::from_seconds(
                init_breath_timer.0 as f32,
                TimerMode::Once,
            )));
        commands.entity(map).remove::<InitBreathTimer>();
    }
}
