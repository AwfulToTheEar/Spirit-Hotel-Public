use crate::game::movement::components::prelude::{
    CanMove, MoveToLocation, MovementDuration, MovementIntent, Pushable,
};
use crate::game::path_finding::components::prelude::ScanCollider;
use crate::game::trigger::components::prelude::TriggerCollider;
use avian2d::collision::collider::contact_query;
use avian2d::prelude::Collider;
use bevy::prelude::*;

pub fn compute_intent(
    commands: &mut Commands,
    entity: Entity,
    transform: &Transform,
    intent: &MovementIntent,
    duration: &MovementDuration,
    step: Vec3,
    children: &Children,
    can_push: bool,
    q_colliders: Query<
        (Entity, &Collider, &GlobalTransform, &ChildOf),
        (Without<TriggerCollider>, Without<ScanCollider>),
    >,
    q_pushables: Query<
        (Entity, &Transform, Has<MoveToLocation>, &Children),
        (With<CanMove>, With<Pushable>),
    >,
    being_pushed: bool,
) -> bool {
    for child in children.iter() {
        let Ok((_, collider, collider_transform, _)) = q_colliders.get(child) else {
            continue;
        };

        for (other_entity, other_collider, other_transform, child_of) in q_colliders.iter() {
            if child == other_entity {
                continue;
            }

            let Ok(result) = contact_query::intersection_test(
                collider,
                (collider_transform.translation() + step).truncate(),
                collider_transform.rotation(),
                other_collider,
                other_transform.translation().truncate(),
                other_transform.rotation(),
            ) else {
                continue;
            };

            if result {
                if (!can_push || can_push && q_pushables.get(child_of.0).is_err()) {
                    commands.entity(entity).remove::<MovementIntent>();
                    return true;
                }

                let (pushable_entity, pushable_transform, has_move_to_location, pushable_children) =
                    q_pushables.get(child_of.0).unwrap();

                if has_move_to_location {
                    commands.entity(entity).remove::<MovementIntent>();
                    return true;
                }

                let obstructed = compute_intent(
                    commands,
                    pushable_entity,
                    pushable_transform,
                    intent,
                    duration,
                    step,
                    pushable_children,
                    can_push,
                    q_colliders,
                    q_pushables,
                    true,
                );

                if obstructed {
                    commands.entity(entity).remove::<MovementIntent>();
                    return true;
                }

                if being_pushed {
                    commands.entity(entity).insert(*intent);
                }

                commands.entity(entity).insert(MoveToLocation {
                    timer: Timer::from_seconds(duration.0, TimerMode::Once),
                    start: transform.translation,
                    end: transform.translation + step,
                });
                return false;
            }
        }
    }

    if being_pushed {
        commands.entity(entity).insert(*intent);
    }

    commands.entity(entity).insert(MoveToLocation {
        timer: Timer::from_seconds(duration.0, TimerMode::Once),
        start: transform.translation,
        end: transform.translation + step,
    });
    return false;
}
