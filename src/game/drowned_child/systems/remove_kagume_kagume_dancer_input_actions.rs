use crate::game::drowned_child::components::prelude::KagumeKagumeDancer;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn remove_kagume_kagume_dancer_input_actions(
    mut commands: Commands,
    mut rc_kagume_kagume_dancers: RemovedComponents<KagumeKagumeDancer>,
) {
    for entity in rc_kagume_kagume_dancers.read() {
        commands
            .entity(entity)
            .despawn_related::<Actions<KagumeKagumeDancer>>();
    }
}
