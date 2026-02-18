use bevy::prelude::*;
use crate::components::prelude::TriggerHeldFor;
use crate::events::prelude::TriggerExited;

pub fn remove_continuous_trigger_duration(
    e_exited_trigger: On<TriggerExited>,
    mut commands: Commands
) {
    commands.entity(e_exited_trigger.trigger).remove::<TriggerHeldFor>();
}