use crate::components::prelude::{BreathTimer, BreathTimerIndicator};
use bevy::prelude::*;

pub fn update_breath_timer_indicator(
    s_breath_timer: Single<(&BreathTimer, Ref<BreathTimer>)>,
    s_breath_timer_indicator: Single<&mut Text, With<BreathTimerIndicator>>,
) {
    let (breath_timer, breath_timer_ref) = s_breath_timer.into_inner();

    if breath_timer_ref.is_added() || !breath_timer_ref.is_changed() {
        return;
    }

    let mut text = s_breath_timer_indicator.into_inner();

    text.0 = format!(
        "Breath: {}",
        breath_timer.0.remaining().as_secs_f32().ceil()
    );
}
