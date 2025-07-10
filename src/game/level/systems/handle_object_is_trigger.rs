use avian2d::prelude::Collider;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledObjectCreated;

use crate::game::trigger::components::prelude::{IsTrigger, TriggerCollider};

pub fn handle_object_is_trigger(
    mut commands: Commands,
    mut e_object_created: EventReader<TiledObjectCreated>,
    q_triggers: Query<&Children, With<IsTrigger>>,
    q_colliders: Query<Entity, (With<Collider>, With<ChildOf>)>,
) {
    for e in e_object_created.read() {
        let Ok(children) = q_triggers.get(e.entity) else {
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
