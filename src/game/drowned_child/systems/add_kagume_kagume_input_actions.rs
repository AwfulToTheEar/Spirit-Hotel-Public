use crate::game::drowned_child::components::prelude::KagumeKagumeDancer;
use crate::game::drowned_child::input::prelude::KagumeKagumeDance;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn add_kagume_kagume_input_actions(
    mut commands: Commands,
    q_kagume_kagume_dancers: Query<Entity, With<KagumeKagumeDancer>>,
) {
    for entity in q_kagume_kagume_dancers.iter() {
        commands.entity(entity).insert((actions!(
            KagumeKagumeDancer[(
                Action::<KagumeKagumeDance>::new(),
                bindings![KeyCode::Space]
            )]
        ),));
    }
}
