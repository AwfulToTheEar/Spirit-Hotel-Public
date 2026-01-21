use crate::input::prelude::KagumeKagumeDance;
use bevy::prelude::*;
use bevy_enhanced_input::action::Action;
use bevy_enhanced_input::prelude::ActionValue;
use spirit_hotel_drowned_child::components::prelude::{KagumeKagumeCycle, KagumeKagumeCycleTicking};

pub fn tick_kagume_kagume_cycle(
    mut commands: Commands,
    time: Res<Time>,
    s_kagume_kagume_cycle: Single<(Entity, &mut KagumeKagumeCycle, Has<KagumeKagumeCycleTicking>)>,
    q_kagume_kagume_dance_value: Query<&ActionValue, With<Action<KagumeKagumeDance>>>,
) {
    let (entity, mut kagume_kagume_cycle, is_ticking) = s_kagume_kagume_cycle.into_inner();

    if is_ticking {
        kagume_kagume_cycle.0.tick(time.delta());
        return;
    }

    let Some(value) = q_kagume_kagume_dance_value.iter().next() else {
        return;
    };

    if !value.as_bool() {
        return;
    }

    commands.entity(entity).insert(KagumeKagumeCycleTicking);
    kagume_kagume_cycle.0.tick(time.delta());
}
