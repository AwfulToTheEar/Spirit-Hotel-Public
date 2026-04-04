use crate::components::prelude::KagumeKagumeDancer;
use bevy::prelude::*;
use spirit_hotel_drowned_child::components::prelude::KagumeKagume;
use spirit_hotel_movement::components::prelude::MovementFlag;

pub fn add_block_movement_kagume_kagume_dancers(
    mut commands: Commands,
    q_kagume_kagume_dancers: Query<Entity, (With<KagumeKagumeDancer>, Without<MovementFlag>)>,
    _: Populated<Entity, With<KagumeKagume>>,
) {
    for entity in q_kagume_kagume_dancers.iter() {
        commands.entity(entity).insert(MovementFlag::Block);
    }
}
