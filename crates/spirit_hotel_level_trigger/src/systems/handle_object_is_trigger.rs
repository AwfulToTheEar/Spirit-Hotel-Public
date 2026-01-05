use spirit_hotel_core::avian::prelude::Collider;
use bevy::prelude::*;
use spirit_hotel_level::tiled::prelude::{ObjectCreated, TiledEvent};
use spirit_hotel_trigger::components::prelude::{IsTrigger, TriggerCollider};

pub fn handle_object_is_trigger(
    mut commands: Commands,
    mut e_object_created: MessageReader<TiledEvent<ObjectCreated>>,
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
