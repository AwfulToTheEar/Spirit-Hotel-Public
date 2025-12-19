use bevy::prelude::*;
use crate::game::movement::components::prelude::{MoveToLocation, MovementCompleted, MovementIntent, SyncMovement};

pub fn mark_movement_completed(
    mut commands: Commands,
    q_timers: Query<(Entity, &MoveToLocation), (With<MovementIntent>, With<SyncMovement>, Without<MovementCompleted>)>,
) {
    for (entity, move_to_location) in q_timers.iter() {
        if !move_to_location.timer.finished() {
            continue;
        }

        commands
            .entity(entity)
            .insert(MovementCompleted);
    }
}