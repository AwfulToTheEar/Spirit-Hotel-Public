use crate::components::prelude::WaitForExternalAction;
use crate::events::prelude::ExternalActionCompleted;
use bevy::prelude::*;

pub fn unlock_trigger_when_external_action_completed(
    e_external_action_completed: On<ExternalActionCompleted>,
    mut commands: Commands,
) {
    commands
        .entity(e_external_action_completed.trigger)
        .remove::<WaitForExternalAction>();
}
