use crate::game::drowned_child::components::prelude::KagumeKagumeDancer;
use crate::game::movement::components::prelude::MovementFlag;
use bevy::prelude::*;

pub fn add_force_movement_kagume_kagume_dancers(
    mut commands: Commands,
    q_kagume_kagume_dancers: Query<Entity, With<KagumeKagumeDancer>>,
) {
    for entity in q_kagume_kagume_dancers.iter() {
        commands.entity(entity).insert(MovementFlag::Force);
    }
}
