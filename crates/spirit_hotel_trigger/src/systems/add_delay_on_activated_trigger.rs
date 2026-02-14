use bevy::prelude::*;
use spirit_hotel_core::fades::{Delay, FadeTask, FadeTasks};
use crate::components::prelude::{AddDelay, FadeInFromTrigger, IsTrigger};
use crate::events::prelude::TriggerActivated;

pub fn add_delay_on_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    mut fade_tasks: ResMut<FadeTasks>,
    q_delay_adders: Query<&AddDelay, (With<IsTrigger>, With<FadeInFromTrigger>)>
) {
    let Ok(add_delay) = q_delay_adders.get(e_trigger_activated.trigger) else {
        return;
    };
    
    commands.insert_resource(FadeTask::<Delay>::new(fade_tasks.take()));
    commands.insert_resource(Delay(Timer::from_seconds(add_delay.0, TimerMode::Once)))
}