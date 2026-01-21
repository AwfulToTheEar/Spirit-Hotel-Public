use bevy::prelude::*;
use bevy_enhanced_input::action::Action;
use bevy_enhanced_input::prelude::ActionValue;
use spirit_hotel_drowned_child::components::prelude::{KagumeKagumeCycle, KagumeKagumeCyclePause, KagumeKagumeCycleTicking};
use crate::input::prelude::KagumeKagumeDance;

pub fn tick_kagume_kagume_cycle_pause(
    mut commands: Commands,
    time: Res<Time>,
    s_kagume_kagume_cycle: Single<(Entity, Option<&mut KagumeKagumeCyclePause>, Has<KagumeKagumeCycleTicking>), With<KagumeKagumeCycle>>,
    q_kagume_kagume_dance_value: Query<&ActionValue, With<Action<KagumeKagumeDance>>>
) {
    let Some(value) = q_kagume_kagume_dance_value.iter().next() else {
        return;
    };

    let (entity, option_kagume_kagume_cycle_pause, is_ticking) = s_kagume_kagume_cycle.into_inner();

    if value.as_bool() && option_kagume_kagume_cycle_pause.is_some() {
        commands.entity(entity).remove::<KagumeKagumeCyclePause>();
        return;
    }

    if value.as_bool() || !is_ticking {
        return;
    }
    
    if option_kagume_kagume_cycle_pause.is_none() {
        commands.entity(entity).insert(KagumeKagumeCyclePause(Timer::from_seconds(1.0, TimerMode::Once)));
        return;
    }
    
    option_kagume_kagume_cycle_pause.unwrap().0.tick(time.delta());
}