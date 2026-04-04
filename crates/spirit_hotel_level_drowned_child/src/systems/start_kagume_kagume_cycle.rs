use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledWorld;
use spirit_hotel_drowned_child::components::prelude::{KagumeKagume, KagumeKagumeCycle};

pub fn start_kagume_kagume_cycle(
    mut commands: Commands,
    s_kagume_kagume: Single<&mut KagumeKagume>,
    s_world: Single<Entity, (With<TiledWorld>, Without<KagumeKagumeCycle>)>,
) {
    let entity = s_world.into_inner();

    let mut kagume_kagume = s_kagume_kagume.into_inner();

    let Some(cycle_time) = kagume_kagume.dance_cycles.pop() else {
        return;
    };

    info!("starting kagume cycle");

    commands
        .entity(entity)
        .insert(KagumeKagumeCycle(Timer::from_seconds(
            cycle_time,
            TimerMode::Once,
        )));
}
