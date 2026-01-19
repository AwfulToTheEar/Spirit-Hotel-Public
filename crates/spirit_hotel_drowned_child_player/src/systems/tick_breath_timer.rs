use crate::input::prelude::HoldBreath;
use bevy::prelude::*;
use bevy_enhanced_input::action::Action;
use bevy_enhanced_input::prelude::ActionValue;
use spirit_hotel_drowned_child::components::prelude::BreathTimer;

pub fn tick_breath_timer(
    time: Res<Time>,
    s_breath_timers: Single<&mut BreathTimer>,
    s_hold_breath_value: Single<&ActionValue, With<Action<HoldBreath>>>,
) {
    let value = s_hold_breath_value.into_inner();

    if !value.as_bool() {
        return;
    }

    let mut breath_timer = s_breath_timers.into_inner();

    breath_timer.0.tick(time.delta());
}
