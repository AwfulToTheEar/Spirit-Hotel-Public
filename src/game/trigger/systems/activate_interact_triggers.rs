use bevy::prelude::*;

use crate::game::trigger::{
    components::prelude::{InsideTriggers, IsTrigger},
    events::prelude::TriggerActivatedEvent,
};

pub fn activate_interact_triggers(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut e_trigger_activated: EventWriter<TriggerActivatedEvent>,
    q_inside_triggers: Query<(Entity, &InsideTriggers)>,
    q_triggers: Query<&IsTrigger>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    for (entity, inside_triggers) in q_inside_triggers.iter() {
        for trigger_entity in inside_triggers.entities.iter() {
            let Ok(trigger) = q_triggers.get(*trigger_entity) else {
                continue;
            };

            if matches!(trigger, IsTrigger::Instant) {
                continue;
            }

            e_trigger_activated.write(TriggerActivatedEvent {
                entity,
                trigger: *trigger_entity,
            });
        }
    }
}
