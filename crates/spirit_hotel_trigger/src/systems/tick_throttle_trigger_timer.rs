use bevy::prelude::*;
use crate::components::prelude::ThrottleTriggerTimer;

pub fn tick_throttle_trigger_timer(
    time: Res<Time>,
    mut q_throttle_trigger_timers: Query<&mut ThrottleTriggerTimer>,
) {
    for mut throttle_trigger_timer in q_throttle_trigger_timers.iter_mut() {
        throttle_trigger_timer.0.tick(time.delta());
    }
}