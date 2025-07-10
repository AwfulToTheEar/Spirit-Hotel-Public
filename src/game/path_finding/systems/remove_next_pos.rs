use bevy::prelude::*;
use bevy_northstar::prelude::{AgentOfGrid, AgentPos, NextPos};

use crate::game::{movement::components::prelude::{MoveToLocation, MovementIntent}, path_finding::components::prelude::NextPosToBeRemoved};

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
            warn!("position not reached");
            continue;
        }

        commands
            .entity(entity)
            .remove::<(NextPos, NextPosToBeRemoved)>();
    }
}
