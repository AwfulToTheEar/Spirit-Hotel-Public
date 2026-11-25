use avian2d::collision::collider::contact_query;
use avian2d::prelude::Collider;
use bevy::prelude::*;
use spirit_hotel_movement::components::prelude::{CanMove, MoveToLocation};
use spirit_hotel_trigger::{
    components::prelude::{InsideTriggers, TriggerCollider},
    events::prelude::{TriggerEntered, TriggerExited},
};

pub fn check_trigger_overlap(
    mut commands: Commands,
    mut q_entites: Query<
        (
            Entity,
            &Collider,
            &GlobalTransform,
            Option<&mut InsideTriggers>,
        ),
        (With<Transform>, Without<MoveToLocation>, With<CanMove>),
    >,
    q_trigger_colliders: Query<
        (Entity, &Collider, &GlobalTransform),
        With<TriggerCollider>,
    >,
) {
    for (entity, collider, transform, mut inside_triggers_option) in q_entites.iter_mut() {
        for (trigger_entity, trigger_collider, trigger_transform) in
            q_trigger_colliders.iter()
        {
            if entity == trigger_entity {
                continue;
            }

            let Ok(result) = contact_query::intersection_test(
                collider,
                transform.translation().truncate(),
                transform.rotation(),
                trigger_collider,
                trigger_transform.translation().truncate(),
                trigger_transform.rotation(),
            ) else {
                continue;
            };

            let Some(inside_triggers) = &mut inside_triggers_option else {
                if !result {
                    continue;
                }

                let mut inside_triggers = InsideTriggers::default();
                inside_triggers.entities.push(trigger_entity);
                commands.entity(entity).insert(inside_triggers);

                commands.trigger(TriggerEntered {
                    by: entity,
                    trigger: trigger_entity,
                });

                continue;
            };

            if !result {
                if !inside_triggers.entities.contains(&trigger_entity) {
                    continue;
                }

                inside_triggers.entities.retain(|e| e != &trigger_entity);

                if inside_triggers.entities.is_empty() {
                    commands.entity(entity).remove::<InsideTriggers>();
                }

                commands.trigger(TriggerExited {
                    by: entity,
                    trigger: trigger_entity,
                });

                continue;
            }

            if inside_triggers.entities.contains(&trigger_entity) {
                continue;
            }

            commands.trigger(TriggerEntered {
                by: entity,
                trigger: trigger_entity,
            });

            inside_triggers.entities.push(trigger_entity);
        }
    }
}
