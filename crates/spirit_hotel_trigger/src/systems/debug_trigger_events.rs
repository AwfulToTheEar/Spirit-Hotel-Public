use bevy::prelude::*;
use crate::messages::prelude::{TriggerEnteredMessage, TriggerExitedMessage};

pub fn debug_trigger_events(
    mut e_trigger_entered: MessageReader<TriggerEnteredMessage>,
    mut e_trigger_exited: MessageReader<TriggerExitedMessage>,
) {
    for e in e_trigger_entered.read() {
        info!("entity {} entered trigger {}", e.entity, e.trigger)
    }

    for e in e_trigger_exited.read() {
        info!("entity {} exited trigger {}", e.entity, e.trigger)
    }
}
