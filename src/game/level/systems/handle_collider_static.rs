use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledColliderCreated, TiledColliderMarker};

use crate::game::movement::components::prelude::CanMove;

pub fn handle_collider_static(
    mut commands: Commands,
    mut e_collider_created: EventReader<TiledColliderCreated>,
    q_static: Query<Entity, Without<CanMove>>,
    q_colliders: Query<(Entity, &ChildOf), With<TiledColliderMarker>>,
) {
    for e in e_collider_created.read() {
        let Ok((entity, child_of)) = q_colliders.get(e.entity) else {
            continue;
        };

        if !q_static.contains(child_of.0) {
            continue;
        }

        commands.entity(entity).insert(RigidBody::Static);
    }
}
