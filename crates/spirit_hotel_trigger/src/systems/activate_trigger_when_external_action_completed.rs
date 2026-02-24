use crate::events::prelude::{ExternalActionCompleted, TriggerActivated};
use bevy::prelude::*;

pub fn activate_trigger_when_external_action_completed(
    e_external_action_completed: On<ExternalActionCompleted>,
    mut commands: Commands,
) {
    commands.trigger(TriggerActivated {
        by: e_external_action_completed.by,
        trigger: e_external_action_completed.trigger,
    })
}
