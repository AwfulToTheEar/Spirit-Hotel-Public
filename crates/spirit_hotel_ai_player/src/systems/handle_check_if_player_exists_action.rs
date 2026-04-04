use crate::components::prelude::{CheckIfPlayerExistsAction, PlayerExists};
use bevy::prelude::*;
use bevy_dogoap::planner::UpdatePlan;
use spirit_hotel_ai::Possible;
use spirit_hotel_player::components::prelude::Player;

pub fn handle_check_if_player_exists_action(
    mut commands: Commands,
    mut q_checking_if_player_exists: Populated<
        (Entity, &mut PlayerExists),
        With<CheckIfPlayerExistsAction>,
    >,
    q_player: Query<Entity, With<Player>>,
) {
    if q_player.is_empty() {
        for (entity, mut can_find_player) in q_checking_if_player_exists.iter_mut() {
            can_find_player.0 = Possible::No;
            commands
                .entity(entity)
                .remove::<CheckIfPlayerExistsAction>();
            commands.trigger(UpdatePlan::from(entity));
        }

        return;
    }

    for (entity, mut can_find_player) in q_checking_if_player_exists.iter_mut() {
        can_find_player.0 = Possible::Yes;
        commands
            .entity(entity)
            .remove::<CheckIfPlayerExistsAction>();
    }
}
