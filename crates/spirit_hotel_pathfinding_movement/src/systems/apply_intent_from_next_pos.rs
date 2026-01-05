use bevy::prelude::*;
use spirit_hotel_pathfinding::northstar::prelude::{AgentOfGrid, AgentPos, NextPos};
use spirit_hotel_movement::components::prelude::{MoveToLocation, MovementIntent};
use crate::components::prelude::{HaltPathfindingIntent, NextPosToBeRemoved};

pub fn apply_intent_from_next_pos(
    mut commands: Commands,
    q_next_positions: Query<
        (Entity, &AgentPos, &NextPos),
        (
            With<AgentOfGrid>,
            Without<MovementIntent>,
            Without<MoveToLocation>,
            Without<NextPosToBeRemoved>,
            Without<HaltPathfindingIntent>,
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
