use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{ColliderCreated, TiledEvent};
use spirit_hotel_trigger::components::prelude::{IsTrigger, TriggerCollider};

pub fn handle_object_is_trigger(
    e_collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
    q_triggers: Query<Entity, With<IsTrigger>>,
) {
    if !q_triggers.contains(e_collider_created.event.collider_of.0) {
        return;
    };

    commands
        .entity(e_collider_created.origin)
        .insert(TriggerCollider);
}
