use bevy::prelude::*;
use spirit_hotel_drowned_child::components::prelude::KagumeKagume;
use spirit_hotel_movement::components::prelude::MovementFlag;
use crate::components::prelude::KagumeKagumeDancer;

pub fn remove_force_movement_kagume_kagume_dancers(
    _: On<Remove, KagumeKagume>,
    mut commands: Commands,
    q_kagume_kagume_dancers: Query<Entity, With<KagumeKagumeDancer>>,
) {
    for entity in q_kagume_kagume_dancers.iter() {
        commands.entity(entity).try_remove::<MovementFlag>();
    }
}
