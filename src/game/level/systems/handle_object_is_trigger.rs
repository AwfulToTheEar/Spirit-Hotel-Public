use avian2d::prelude::Collider;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{ObjectCreated, TiledEvent};

use crate::game::trigger::components::prelude::{IsTrigger, TriggerCollider};

pub fn handle_object_is_trigger(
    mut commands: Commands,
    mut e_object_created: EventReader<TiledEvent<ObjectCreated>>,
    q_triggers: Query<&Children, With<IsTrigger>>,
    q_colliders: Query<Entity, (With<Collider>, With<ChildOf>)>,
) {
    for e in e_object_created.read() {
        let Ok(children) = q_triggers.get(e.origin) else {
            continue;
        };

        for entity in children.iter() {
            if !q_colliders.contains(entity) {
                continue;
            }

            commands.entity(entity).insert(TriggerCollider);
        }
    }
}
