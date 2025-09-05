use crate::game::drowned_child::components::prelude::KagumeKagumeDancer;
use crate::game::movement::components::prelude::MovementFlag;
use bevy::prelude::*;

pub fn add_block_movement_kagume_kagume_dancers(
    mut commands: Commands,
    q_kagume_kagume_dancers: Query<Entity, (With<KagumeKagumeDancer>, Without<MovementFlag>)>,
) {
    for entity in q_kagume_kagume_dancers.iter() {
        commands.entity(entity).insert(MovementFlag::Block);
    }
}
