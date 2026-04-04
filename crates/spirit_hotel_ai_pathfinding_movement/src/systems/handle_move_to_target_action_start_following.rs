use crate::components::prelude::MoveToTargetAction;
use bevy::prelude::*;
use bevy_northstar::prelude::AgentOfGrid;
use spirit_hotel_ai::components::prelude::Target;
use spirit_hotel_movement::components::prelude::CanMove;
use spirit_hotel_pathfinding_movement::components::prelude::Follow;

pub fn handle_move_to_target_action_start_following(
    mut commands: Commands,
    q_moving_to_target: Populated<
        (Entity, &Target, &AgentOfGrid),
        (With<MoveToTargetAction>, With<CanMove>),
    >,
    q_targets: Query<&AgentOfGrid>,
) {
    for (entity, target, agent_of_grid) in q_moving_to_target.iter() {
        let Ok(other_agent_of_grid) = q_targets.get(target.0) else {
            panic!("Target entity not found");
        };

        if agent_of_grid.0 != other_agent_of_grid.0 {
            panic!("Agent of grid mismatch");
        }

        commands.entity(entity).insert(Follow(target.0));
    }
}
