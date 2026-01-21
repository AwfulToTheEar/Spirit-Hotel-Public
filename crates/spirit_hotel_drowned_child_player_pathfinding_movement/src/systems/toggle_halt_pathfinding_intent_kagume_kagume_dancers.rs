use bevy::prelude::*;
use bevy_enhanced_input::prelude::{ActionOf, ActionValue};
use spirit_hotel_drowned_child::components::prelude::KagumeKagume;
use spirit_hotel_drowned_child_movement::components::prelude::KagumeKagumeDancer;
use spirit_hotel_pathfinding_movement::components::prelude::HaltPathfindingIntent;

pub fn toggle_halt_pathfinding_intent_kagume_kagume_dancers(
    mut commands: Commands,
    q_kagume_kagume_dance_actions: Query<(&ActionValue, &ActionOf<KagumeKagumeDancer>)>,
    q_kagume_kagume: Query<Entity, With<KagumeKagume>>
) {
    if q_kagume_kagume.is_empty() {
        return;
    }

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
