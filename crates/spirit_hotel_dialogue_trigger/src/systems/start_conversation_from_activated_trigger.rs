use bevy::prelude::*;
use spirit_hotel_dialogue::components::prelude::StartConversation;
use bevy_yarnspinner::prelude::DialogueRunner;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::messages::prelude::TriggerActivatedMessage;

pub fn start_conversation_from_activated_trigger(
    mut e_trigger_activated: MessageReader<TriggerActivatedMessage>,
    q_conversation_starters: Query<&StartConversation, With<IsTrigger>>,
    mut q_dialogue_runner: Query<&mut DialogueRunner>,
) {
    let Ok(mut dialogue_runner) = q_dialogue_runner.single_mut() else {
        return;
    };

    if dialogue_runner.is_running() {
        return;
    }

    for e in e_trigger_activated.read() {
        let Ok(start_conversation) = q_conversation_starters.get(e.trigger) else {
            continue;
        };

        if !dialogue_runner.node_exists(&start_conversation.0) {
            continue;
        }

        dialogue_runner.start_node(start_conversation.0.clone());

        return;
    }
}
