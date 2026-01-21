use bevy::prelude::*;
use crate::components::prelude::{KagumeKagumeCycle, KagumeKagumeCycleIndicator};

pub fn update_kagume_kagume_cycle_indicator(
    s_kagume_kagume_cycle: Single<(&KagumeKagumeCycle, Ref<KagumeKagumeCycle>)>,
    s_kagume_kagume_cycle_indicator: Single<&mut Text, With<KagumeKagumeCycleIndicator>>,
) {
    let (kagume_kagume_cycle, kagume_kagume_cycle_ref) = s_kagume_kagume_cycle.into_inner();

    if kagume_kagume_cycle_ref.is_added() || !kagume_kagume_cycle_ref.is_changed() {
        return;
    }

    let mut text = s_kagume_kagume_cycle_indicator.into_inner();

    text.0 = format!(
        "Dance Time: {}",
        kagume_kagume_cycle.0.remaining().as_secs_f32().ceil()
    );
}
