use bevy::prelude::*;
use crate::components::prelude::{CheckMovementIntent, MoveToLocation, MovementCompleted, MovementIntent, SyncMovement, SyncMovementWith};

pub fn remove_move_to_location_synced<C: Component>(
    mut commands: Commands,
    q_synced_entities: Query<(Entity, Has<MovementCompleted>), (With<MovementIntent>, With<SyncMovementWith<C>>, With<SyncMovement>)>,
) {
    if q_synced_entities.iter().find(|(_, movement_completed)| !movement_completed).is_some() {
        return;
    }

    for (entity, _) in q_synced_entities.iter() {
        commands
            .entity(entity)
            .insert(CheckMovementIntent)
            .remove::<(MoveToLocation, MovementCompleted)>();
    }
}