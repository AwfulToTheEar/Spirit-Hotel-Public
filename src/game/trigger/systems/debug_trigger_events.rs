use bevy::prelude::*;

use crate::game::trigger::events::prelude::{TriggerEnteredEvent, TriggerExitedEvent};

pub fn debug_trigger_events(
    mut e_trigger_entered: EventReader<TriggerEnteredEvent>,
    mut e_trigger_exited: EventReader<TriggerExitedEvent>,
) {
    for e in e_trigger_entered.read() {
        info!("entity {} entered trigger {}", e.entity, e.trigger)
    }

    for e in e_trigger_exited.read() {
        info!("entity {} exited trigger {}", e.entity, e.trigger)
    }
}
