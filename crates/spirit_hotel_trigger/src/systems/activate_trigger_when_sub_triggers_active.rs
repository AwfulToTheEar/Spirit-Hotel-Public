use crate::components::prelude::{
    ActivateWhenSubTriggersActive, SubTriggerOf, SubTriggers, TriggerHasBeenActivated,
    TriggerHeldFor,
};
use crate::events::prelude::TriggerActivated;
use bevy::prelude::*;

pub fn activate_trigger_when_sub_triggers_active(
    mut commands: Commands,
    q_triggers: Query<
        (Entity, &SubTriggers),
        (
            With<ActivateWhenSubTriggersActive>,
            Without<TriggerHasBeenActivated>,
        ),
    >,
    q_sub_triggers: Query<(
        &SubTriggerOf,
        Has<TriggerHasBeenActivated>,
        Has<TriggerHeldFor>,
    )>,
) {
    for (trigger_entity, sub_triggers) in q_triggers.iter() {
        let mut sub_triggers_not_valid = false;
        for sub_trigger in sub_triggers.iter() {
            let Ok((sub_trigger_of, has_trigger_been_activated, has_trigger_held_for)) =
                q_sub_triggers.get(sub_trigger)
            else {
                continue;
            };

            if !has_trigger_been_activated {
                sub_triggers_not_valid = true;
                break;
            }

            if sub_trigger_of.should_be_held && !has_trigger_held_for {
                sub_triggers_not_valid = true;
                break;
            }
        }

        if sub_triggers_not_valid {
            continue;
        }

        commands.trigger(TriggerActivated {
            by: None,
            trigger: trigger_entity,
        });
    }
}
