use crate::components::prelude::Brain;
use bevy::prelude::*;
use bevy::platform as bevy_platform;
use bevy_dogoap::prelude::*;
use spirit_hotel_ai::Possible;
use spirit_hotel_ai::components::prelude::HasTarget;
use spirit_hotel_ai_pathfinding::components::prelude::{CanReachTarget, CheckIfTargetReachableAction, TargetInReach};
use spirit_hotel_ai_pathfinding_movement::components::prelude::MoveToTargetAction;
use spirit_hotel_ai_player::components::prelude::{
    PlayerExists, CheckIfPlayerExistsAction, FindPlayerAction,
};

pub fn setup_goap_for_brain(
    e_brain_added: On<Add, Brain>,
    mut commands: Commands,
    q_brains: Query<(Entity, &Brain)>,
) {
    let Ok((entity, brain)) = q_brains.get(e_brain_added.entity) else {
        return;
    };

    match brain {
        Brain::PlayerFollower => {
            let goal = Goal::from_reqs(&[TargetInReach::is(true)]);

            let check_if_player_exists_action = CheckIfPlayerExistsAction::action()
                .with_precondition(PlayerExists::is(Possible::Unknown))
                .with_mutator(PlayerExists::set(Possible::Yes));

            let find_player_action = FindPlayerAction::action()
                .with_precondition(PlayerExists::is(Possible::Yes))
                .with_mutator(HasTarget::set(true));

            let check_if_target_reachable_action = CheckIfTargetReachableAction::action()
                .with_precondition(HasTarget::is(true))
                .with_precondition(CanReachTarget::is(Possible::Unknown))
                .with_mutator(CanReachTarget::set(Possible::Yes));

            let move_to_target_action = MoveToTargetAction::action()
                .with_precondition(HasTarget::is(true))
                .with_precondition(CanReachTarget::is(Possible::Yes))
                .with_mutator(TargetInReach::set(true));

            let (planner, components) = create_planner!({
                actions: [
                    (CheckIfPlayerExistsAction, check_if_player_exists_action),
                    (FindPlayerAction, find_player_action),
                    (CheckIfTargetReachableAction, check_if_target_reachable_action),
                    (MoveToTargetAction, move_to_target_action)
                ],
                state: [HasTarget(false), PlayerExists(Possible::Unknown), CanReachTarget(Possible::Unknown), TargetInReach(false)],
                goals: [goal],
            });

            commands.entity(entity).insert((planner, components));
            commands.trigger(UpdatePlan::from(entity))
        }
    }
}
