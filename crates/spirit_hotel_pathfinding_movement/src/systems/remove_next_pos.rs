use bevy::prelude::*;
use spirit_hotel_pathfinding::northstar::prelude::{AgentOfGrid, AgentPos, NextPos};

use spirit_hotel_movement::components::prelude::{MoveToLocation, MovementIntent};
use crate::components::prelude::NextPosToBeRemoved;

pub fn remove_next_pos(
    mut commands: Commands,
    q_next_positions: Query<
        (Entity, &AgentPos, &NextPos),
        (
            With<AgentOfGrid>,
            Without<MovementIntent>,
            Without<MoveToLocation>,
            With<NextPosToBeRemoved>,
        ),
    >,
) {
    for (entity, grid_pos, next_pos) in q_next_positions.iter() {
        if grid_pos.0 != next_pos.0 {
            //TODO: do something when position can't be reached
            warn!("position not reached (at: {}, goal: {})", grid_pos.0, next_pos.0);
            continue;
        }

        commands
            .entity(entity)
            .remove::<(NextPos, NextPosToBeRemoved)>();
    }
}
