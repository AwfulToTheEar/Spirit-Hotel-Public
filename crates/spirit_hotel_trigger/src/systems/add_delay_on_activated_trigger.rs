use crate::components::prelude::{AddDelay, FromTrigger, IsTrigger};
use crate::events::prelude::TriggerActivated;
use bevy::prelude::*;
use spirit_hotel_core::fades::{Delay, FadeTask, FadeTasks};

pub fn add_delay_on_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    mut fade_tasks: ResMut<FadeTasks>,
    q_delay_adders: Query<(&AddDelay, &FromTrigger), With<IsTrigger>>,
) {
    let Ok((add_delay, from_trigger)) = q_delay_adders.get(e_trigger_activated.trigger) else {
        return;
    };

    if !matches!(from_trigger, FromTrigger::FadeIn) {
        return;
    }

    commands.insert_resource(FadeTask::<Delay>::new(fade_tasks.take()));
    commands.insert_resource(Delay(Timer::from_seconds(add_delay.0, TimerMode::Once)))
}
