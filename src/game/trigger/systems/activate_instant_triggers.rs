use bevy::prelude::*;

use crate::game::trigger::{
    components::prelude::IsTrigger,
    events::prelude::{TriggerActivatedEvent, TriggerEnteredEvent},
};

pub fn activate_instant_triggers(
    mut e_trigger_entered: EventReader<TriggerEnteredEvent>,
    mut e_trigger_activated: EventWriter<TriggerActivatedEvent>,
    q_triggers: Query<&IsTrigger>,
) {
    for e in e_trigger_entered.read() {
        let Ok(trigger) = q_triggers.get(e.trigger) else {
            continue;
        };

        if matches!(trigger, IsTrigger::Interact) {
            return;
        }

        e_trigger_activated.write(TriggerActivatedEvent {
            entity: e.entity,
            trigger: e.trigger,
        });
    }
}
