use crate::game::drowned_child::components::prelude::BreathTimer;
use bevy::prelude::*;

pub fn tick_breath_timer(time: Res<Time>, mut q_breath_timers: Query<&mut BreathTimer>) {
    for mut breath_timer in q_breath_timers.iter_mut() {
        breath_timer.0.tick(time.delta());
    }
}
