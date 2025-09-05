use crate::game::drowned_child::components::prelude::KagumeKagumeDancer;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn remove_kagume_kagume_input_actions(
    mut commands: Commands,
    q_kagume_kagume_dancers: Query<Entity, With<KagumeKagumeDancer>>,
) {
    for entity in q_kagume_kagume_dancers.iter() {
        commands
            .entity(entity)
            .despawn_related::<Actions<KagumeKagumeDancer>>();
    }
}
