use crate::components::prelude::MoveToTargetAction;
use bevy::prelude::*;
use spirit_hotel_ai_pathfinding::components::prelude::TargetInReach;
use spirit_hotel_pathfinding_movement::components::prelude::CollidedWithFollowTarget;

pub fn handle_move_to_target_action_target_reached(
    mut commands: Commands,
    mut q_moving_to_target: Query<
        (Entity, &mut TargetInReach),
        (With<MoveToTargetAction>, Added<CollidedWithFollowTarget>),
    >,
) {
    for (entity, mut target_in_reach) in q_moving_to_target.iter_mut() {
        target_in_reach.0 = true;
        commands.entity(entity).remove::<MoveToTargetAction>();
    }
}
