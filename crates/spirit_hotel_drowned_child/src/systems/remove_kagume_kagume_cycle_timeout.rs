use crate::components::prelude::{
    KagumeKagumeCycle, KagumeKagumeCycleIndicator, KagumeKagumeCyclePause, KagumeKagumeCycleTicking,
};
use bevy::prelude::*;

pub fn remove_kagume_kagume_cycle_timeout(
    mut commands: Commands,
    s_kagume_kagume_cycle: Single<(Entity, &KagumeKagumeCycle)>,
    s_kagume_kagume_cycle_indicator: Single<Entity, With<KagumeKagumeCycleIndicator>>,
) {
    let (kagume_kagume_cycle_entity, kagume_kagume_cycle) = s_kagume_kagume_cycle.into_inner();
    let kagume_kagume_cycle_indicator_entity = s_kagume_kagume_cycle_indicator.into_inner();

    if kagume_kagume_cycle.0.just_finished() {
        commands.entity(kagume_kagume_cycle_entity).remove::<(
            KagumeKagumeCycle,
            KagumeKagumeCyclePause,
            KagumeKagumeCycleTicking,
        )>();
        commands
            .entity(kagume_kagume_cycle_indicator_entity)
            .despawn();
    }
}
