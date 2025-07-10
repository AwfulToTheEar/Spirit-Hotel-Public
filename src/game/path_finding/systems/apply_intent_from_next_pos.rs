use bevy::prelude::*;
use bevy_northstar::prelude::{AgentOfGrid, AgentPos, NextPos};

use crate::game::{movement::components::prelude::{MoveToLocation, MovementIntent}, path_finding::components::prelude::NextPosToBeRemoved};

pub fn apply_intent_from_next_pos(
    mut commands: Commands,
    q_next_positions: Query<
        (Entity, &AgentPos, &NextPos),
        (
            With<AgentOfGrid>,
            Without<MovementIntent>,
            Without<MoveToLocation>,
            Without<NextPosToBeRemoved>,
        ),
    >,
) {
    for (entity, grid_pos, next_pos) in q_next_positions.iter() {
        commands.entity(entity).insert((
            MovementIntent(
                (next_pos.0.as_vec3() - grid_pos.0.as_vec3())
                    .truncate()
                    .normalize(),
            ),
            NextPosToBeRemoved,
        ));
    }
}
