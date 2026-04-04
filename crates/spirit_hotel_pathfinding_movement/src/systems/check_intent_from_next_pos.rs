use crate::components::prelude::{NextPosToBeRemoved};
use bevy::prelude::*;
use bevy_northstar::components::{AgentOfGrid, NextPos};
use bevy_northstar::prelude::Path;
use spirit_hotel_movement::components::prelude::{
    CheckMovementIntent, MoveToLocation, MovementIntent, UpdateMovementIntent,
};

pub fn check_intent_from_next_pos(
    mut commands: Commands,
    q_next_positions: Query<
        (Entity, Has<NextPos>, Has<Path>),
        (
            With<AgentOfGrid>,
            With<CheckMovementIntent>,
            Without<MoveToLocation>,
            Without<NextPosToBeRemoved>,
        ),
    >,
) {
    for (entity, has_next_pos, has_path) in q_next_positions.iter() {
        if has_next_pos {
            commands
                .entity(entity)
                .remove::<CheckMovementIntent>()
                .insert(UpdateMovementIntent);
            continue;
        }

        if has_path {
            continue;
        }

        commands.entity(entity).remove::<(MovementIntent, CheckMovementIntent)>();
    }
}
