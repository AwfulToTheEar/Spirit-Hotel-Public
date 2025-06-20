use avian2d::{collision::collider::contact_query, prelude::Collider};
use bevy::prelude::*;

use crate::{game::{movement::{CanMove, MoveToLocation}, GameplaySystems}, AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app
    .register_type::<IsTrigger>()

    .add_event::<TriggerEnteredEvent>()
    .add_event::<TriggerExitedEvent>()
    .add_event::<TriggerActivatedEvent>()

    .add_systems(Update, (
            check_trigger_overlap,
            debug_trigger_events,
            activate_instant_triggers,
            activate_interact_triggers,
        )
            .in_set(AppSystems::Update)
            .in_set(GameplaySystems)
            .in_set(PausableSystems)
    );
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
pub enum IsTrigger {
    #[default]
    Instant,
    Interact
}

#[derive(Component)]
pub struct TriggerCollider;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct InsideTriggers {
    pub entities: Vec<Entity>
}

#[derive(Event)]
pub struct TriggerEnteredEvent {
    pub entity: Entity,
    pub trigger: Entity
}

#[derive(Event)]
pub struct TriggerExitedEvent {
    pub entity: Entity,
    pub trigger: Entity
}

#[derive(Event)]
pub struct TriggerActivatedEvent {
    pub entity: Entity,
    pub trigger: Entity
}

pub fn check_trigger_overlap(
    mut commands: Commands,
    mut e_trigger_entered: EventWriter<TriggerEnteredEvent>,
    mut e_trigger_exited: EventWriter<TriggerExitedEvent>,
    mut q_entites: Query<(Entity, &Children, Option<&mut InsideTriggers>), (With<Transform>, Without<MoveToLocation>, With<CanMove>)>,
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
                    trigger_transform.rotation()
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
                        entity: entity,
                        trigger: child_of.0
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

                    e_trigger_exited.write(TriggerExitedEvent{
                        entity: entity,
                        trigger: child_of.0
                    });

                    continue;
                }

                if inside_triggers.entities.contains(&child_of.0) {
                    continue;
                }

                e_trigger_entered.write(TriggerEnteredEvent {
                    entity: entity,
                    trigger: child_of.0
                });

                inside_triggers.entities.push(child_of.0);
            }
        }
    }
}

pub fn debug_trigger_events(
    mut e_trigger_entered: EventReader<TriggerEnteredEvent>,
    mut e_trigger_exited: EventReader<TriggerExitedEvent>
) {

    for e in e_trigger_entered.read() {
        info!("entity {} entered trigger {}", e.entity, e.trigger)
    }

    for e in e_trigger_exited.read() {
        info!("entity {} exited trigger {}", e.entity, e.trigger)
    }
}

pub fn activate_instant_triggers(
    mut e_trigger_entered: EventReader<TriggerEnteredEvent>,
    mut e_trigger_activated: EventWriter<TriggerActivatedEvent>,
    q_triggers: Query<&IsTrigger>
) {
    for e in e_trigger_entered.read() {
        let Ok(trigger) = q_triggers.get(e.trigger) else {
            continue;
        };

        if match trigger {
            IsTrigger::Interact => true,
            _ => false
        } {
            return;
        }

        e_trigger_activated.write(TriggerActivatedEvent { entity: e.entity, trigger: e.trigger });
    }
}

pub fn activate_interact_triggers(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut e_trigger_activated: EventWriter<TriggerActivatedEvent>,
    q_inside_triggers: Query<(Entity, &InsideTriggers)>,
    q_triggers: Query<&IsTrigger>
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    for (entity, inside_triggers) in q_inside_triggers.iter() {
        for trigger_entity in inside_triggers.entities.iter() {
            let Ok(trigger) = q_triggers.get(trigger_entity.clone()) else {
                continue;
            };

            if match trigger {
                IsTrigger::Instant => true,
                _ => false
            } {
                continue;
            }

            e_trigger_activated.write(TriggerActivatedEvent { entity: entity, trigger: trigger_entity.clone() });
        }
    }
}
