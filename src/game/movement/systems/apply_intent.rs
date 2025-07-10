use avian2d::{collision::collider::contact_query, prelude::Collider};
use bevy::prelude::*;

use crate::game::{
    movement::components::prelude::{CanMove, MoveToLocation, MovementDuration, MovementIntent},
    path_finding::components::prelude::{GridTileSize, ScanCollider},
    trigger::components::prelude::TriggerCollider,
};

//TODO: support multiple grids
pub fn apply_intent(
    mut commands: Commands,
    s_grid_tile_size: Single<&GridTileSize>,
    q_entities_with_intent: Query<
        (
            Entity,
            &MovementIntent,
            &MovementDuration,
            &Transform,
            &Children,
        ),
        (With<CanMove>, Without<MoveToLocation>),
    >,
    q_colliders: Query<
        (Entity, &Collider, &GlobalTransform),
        (Without<TriggerCollider>, Without<ScanCollider>),
    >,
) {
    let grid_tile_size = s_grid_tile_size.into_inner();

    for (entity, intent, duration, transform, children) in q_entities_with_intent.iter() {
        let step = (intent.0 * grid_tile_size.0.as_vec2()).extend(0.);

        let mut collider_found = false;
        for child in children.iter() {
            let Ok((_, collider, collider_transform)) = q_colliders.get(child) else {
                continue;
            };

            for (other_entity, other_collider, other_transform) in q_colliders.iter() {
                if children.contains(&other_entity) {
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
                    collider_found = true;
                    commands.entity(entity).remove::<MovementIntent>();
                    break;
                }
            }
        }

        if collider_found {
            continue;
        }

        commands.entity(entity).insert(MoveToLocation {
            timer: Timer::from_seconds(duration.0, TimerMode::Once),
            start: transform.translation,
            end: transform.translation + step,
        });
    }
}
