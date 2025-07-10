use bevy::prelude::*;

use crate::game::movement::components::prelude::{MoveToLocation, MovementIntent};

pub fn remove_move_to_location(
    mut commands: Commands,
    q_timers: Query<(Entity, &MoveToLocation), With<MovementIntent>>,
) {
    for (entity, move_to_location) in q_timers.iter() {
        if !move_to_location.timer.finished() {
            continue;
        }

        commands
            .entity(entity)
            .remove::<(MovementIntent, MoveToLocation)>();
    }
}
