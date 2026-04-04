use crate::components::prelude::{CanReachTarget, CheckIfTargetReachableAction};
use bevy::prelude::*;
use bevy_dogoap::planner::UpdatePlan;
use bevy_northstar::grid::Grid;
use bevy_northstar::neighbor::CardinalNeighborhood;
use bevy_northstar::prelude::{AgentOfGrid, AgentPos};
use spirit_hotel_ai::Possible;
use spirit_hotel_ai::components::prelude::Target;

pub fn handle_check_if_target_reachable_action(
    mut commands: Commands,
    mut q_checking_if_target_reachable: Populated<
        (
            Entity,
            &mut CanReachTarget,
            &Target,
            &AgentOfGrid,
            &AgentPos,
        ),
        With<CheckIfTargetReachableAction>,
    >,
    q_targets: Query<(&AgentOfGrid, &AgentPos)>,
    q_grids: Query<&Grid<CardinalNeighborhood>>,
) {
    for (entity, mut can_reach_target, target, agent_of_grid, agent_pos) in
        q_checking_if_target_reachable.iter_mut()
    {
        let Ok((other_agent_of_grid, other_agent_pos)) = q_targets.get(target.0) else {
            can_reach_target.0 = Possible::No;
            commands
                .entity(entity)
                .remove::<CheckIfTargetReachableAction>();
            commands.trigger(UpdatePlan::from(entity));
            continue;
        };

        if agent_of_grid.0 != other_agent_of_grid.0 {
            can_reach_target.0 = Possible::No;
            commands
                .entity(entity)
                .remove::<CheckIfTargetReachableAction>();
            commands.trigger(UpdatePlan::from(entity));
            continue;
        }

        let Ok(grid) = q_grids.get(agent_of_grid.0) else {
            can_reach_target.0 = Possible::No;
            commands
                .entity(entity)
                .remove::<CheckIfTargetReachableAction>();
            commands.trigger(UpdatePlan::from(entity));
            continue;
        };

        if !grid.is_path_viable(agent_pos.0, other_agent_pos.0) {
            can_reach_target.0 = Possible::No;
            commands
                .entity(entity)
                .remove::<CheckIfTargetReachableAction>();
            commands.trigger(UpdatePlan::from(entity));
            continue;
        }

        can_reach_target.0 = Possible::Yes;
        commands
            .entity(entity)
            .remove::<CheckIfTargetReachableAction>();
    }
}
