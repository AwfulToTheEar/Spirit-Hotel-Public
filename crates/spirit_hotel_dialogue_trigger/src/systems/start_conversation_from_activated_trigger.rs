use bevy::prelude::*;
use bevy_yarnspinner::prelude::DialogueRunner;
use spirit_hotel_dialogue::components::prelude::StartConversation;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::events::prelude::TriggerActivated;

pub fn start_conversation_from_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    q_conversation_starters: Query<&StartConversation, With<IsTrigger>>,
    mut q_dialogue_runner: Query<&mut DialogueRunner>,
) {
    let Ok(mut dialogue_runner) = q_dialogue_runner.single_mut() else {
        return;
    };

    if dialogue_runner.is_running() {
        return;
    }

    let Ok(start_conversation) = q_conversation_starters.get(e_trigger_activated.trigger) else {
        return;
    };

    if !dialogue_runner.node_exists(&start_conversation.0) {
        return;
    }

    dialogue_runner.start_node(start_conversation.0.clone());

    return;
}
