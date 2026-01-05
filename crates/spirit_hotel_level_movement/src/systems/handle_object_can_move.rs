use spirit_hotel_core::avian::prelude::RigidBody;
use bevy::prelude::*;
use spirit_hotel_level::tiled::prelude::{ColliderCreated, TiledColliderSource, TiledEvent};
use spirit_hotel_movement::components::prelude::CanMove;

pub fn handle_object_can_move(
    mut commands: Commands,
    mut e_collider_created: MessageReader<TiledEvent<ColliderCreated>>,
    q_static: Query<Entity, With<CanMove>>,
    q_colliders: Query<(Entity, &ChildOf), With<TiledColliderSource>>,
) {
    for e in e_collider_created.read() {
        let Ok((entity, child_of)) = q_colliders.get(e.origin) else {
            continue;
        };

        if !q_static.contains(child_of.0) {
            continue;
        }

        commands.entity(entity).insert(RigidBody::Kinematic);
    }
}
