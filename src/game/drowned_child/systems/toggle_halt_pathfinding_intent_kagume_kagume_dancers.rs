use crate::game::drowned_child::components::prelude::KagumeKagumeDancer;
use crate::game::path_finding::components::prelude::HaltPathfindingIntent;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::{ActionOf, ActionValue};

pub fn toggle_halt_pathfinding_intent_kagume_kagume_dancers(
    mut commands: Commands,
    q_kagume_kagume_dance_actions: Query<(&ActionValue, &ActionOf<KagumeKagumeDancer>)>,
) {
    for (action_value, action_of) in q_kagume_kagume_dance_actions.iter() {
        if action_value.as_bool() {
            commands
                .entity(action_of.entity())
                .remove::<HaltPathfindingIntent>();
            continue;
        }

        commands
            .entity(action_of.entity())
            .insert(HaltPathfindingIntent);
    }
}
