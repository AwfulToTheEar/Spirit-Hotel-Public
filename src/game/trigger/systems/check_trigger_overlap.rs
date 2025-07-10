use avian2d::{collision::collider::contact_query, prelude::Collider};
use bevy::prelude::*;

use crate::game::{
    movement::components::prelude::{CanMove, MoveToLocation},
    trigger::{
        components::prelude::{InsideTriggers, TriggerCollider},
        events::prelude::{TriggerEnteredEvent, TriggerExitedEvent},
    },
};

pub fn check_trigger_overlap(
    mut commands: Commands,
    mut e_trigger_entered: EventWriter<TriggerEnteredEvent>,
    mut e_trigger_exited: EventWriter<TriggerExitedEvent>,
    mut q_entites: Query<
        (Entity, &Children, Option<&mut InsideTriggers>),
        (With<Transform>, Without<MoveToLocation>, With<CanMove>),
    >,
    q_colliders: Query<(&Collider, &GlobalTransform)>,
    q_trigger_colliders: Query<(&Collider, &GlobalTransform, &ChildOf), With<TriggerCollider>>,
) {
    for (entity, children, mut inside_triggers_option) in q_entites.iter_mut() {
        for child in children.iter() {
            let Ok((collider, collider_transform)) = q_colliders.get(child) else {
                continue;
            };

            for (trigger_collider, trigger_transform, child_of) in q_trigger_colliders.iter() {
                let Ok(result) = contact_query::intersection_test(
                    collider,
                    collider_transform.translation().truncate(),
                    collider_transform.rotation(),
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
                    inside_triggers.entities.push(child_of.0);
                    commands.entity(entity).insert(inside_triggers);

                    e_trigger_entered.write(TriggerEnteredEvent {
                        entity,
                        trigger: child_of.0,
                    });

                    continue;
                };

                if !result {
                    if !inside_triggers.entities.contains(&child_of.0) {
                        continue;
                    }

                    inside_triggers.entities.retain(|e| e != &child_of.0);

                    if inside_triggers.entities.is_empty() {
                        commands.entity(entity).remove::<InsideTriggers>();
                    }

                    e_trigger_exited.write(TriggerExitedEvent {
                        entity,
                        trigger: child_of.0,
                    });

                    continue;
                }

                if inside_triggers.entities.contains(&child_of.0) {
                    continue;
                }

                e_trigger_entered.write(TriggerEnteredEvent {
                    entity,
                    trigger: child_of.0,
                });

                inside_triggers.entities.push(child_of.0);
            }
        }
    }
}
