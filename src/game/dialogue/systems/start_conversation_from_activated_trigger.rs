use bevy::prelude::*;
use bevy_yarnspinner::prelude::DialogueRunner;

use crate::game::{
    dialogue::components::prelude::StartConversation,
    trigger::{components::prelude::IsTrigger, events::prelude::TriggerActivatedEvent},
};

pub fn start_conversation_from_activated_trigger(
    mut e_trigger_activated: EventReader<TriggerActivatedEvent>,
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
