use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{ColliderCreated, TiledEvent};
use spirit_hotel_movement::components::prelude::CanMove;

pub fn handle_object_can_move(
    e_collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
    q_moving: Query<Entity, With<CanMove>>,
) {
    if !q_moving.contains(e_collider_created.event.collider_of.0) {
        return;
    }

    commands
        .entity(e_collider_created.origin)
        .insert(RigidBody::Kinematic);
}
