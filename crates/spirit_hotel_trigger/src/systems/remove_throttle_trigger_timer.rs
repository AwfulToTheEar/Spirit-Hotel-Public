use crate::components::prelude::ThrottleTriggerTimer;
use bevy::prelude::*;

pub fn remove_throttle_trigger_timer(
    mut commands: Commands,
    q_throttle_trigger_timers: Query<(Entity, &ThrottleTriggerTimer)>,
) {
    for (entity, throttle_trigger_timer) in q_throttle_trigger_timers.iter() {
        if throttle_trigger_timer.0.just_finished() {
            commands.entity(entity).remove::<ThrottleTriggerTimer>();
        }
    }
}
