use crate::game::drowned_child::components::prelude::{BreathTimer, BreathTimerIndicator};
use bevy::prelude::*;

pub fn update_breath_timer_indicator(
    s_breath_timer: Single<&BreathTimer, Changed<BreathTimer>>,
    mut s_breath_timer_indicator: Single<(&mut Text), (With<BreathTimerIndicator>)>,
) {
    let breath_timer = s_breath_timer.into_inner();
    let mut text = s_breath_timer_indicator.into_inner();

    text.0 = format!("Breath: {}", breath_timer.0.remaining().as_secs());
}
