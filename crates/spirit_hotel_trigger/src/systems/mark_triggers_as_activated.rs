use bevy::prelude::*;
use crate::components::prelude::TriggerHasBeenActivated;
use crate::events::prelude::TriggerActivated;

pub fn mark_triggers_as_activated(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands
) {
    commands.entity(e_trigger_activated.trigger).insert(TriggerHasBeenActivated);
}