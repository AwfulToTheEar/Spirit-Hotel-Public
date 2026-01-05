use spirit_hotel_core::avian::prelude::Collider;
use bevy::prelude::*;
use spirit_hotel_movement::components::prelude::{CanMove, CanPush, MoveToLocation, MovementDuration, MovementFlag, MovementIntent, Pushable};
use spirit_hotel_pathfinding::components::prelude::{GridTileSize, ScanCollider};
use spirit_hotel_trigger::components::prelude::TriggerCollider;
use crate::utils::prelude::compute_intent;

//TODO: support multiple grids
pub fn apply_intent(
    mut commands: Commands,
    s_grid_tile_size: Single<&GridTileSize>,
    q_entities_with_intent: Query<
        (
            Entity,
            &MovementIntent,
            &MovementDuration,
            Has<CanPush>,
            Option<&MovementFlag>,
            &Transform,
            &Children,
        ),
        (With<CanMove>, Without<MoveToLocation>),
    >,
    q_colliders: Query<
        (Entity, &Collider, &GlobalTransform, &ChildOf),
        (Without<TriggerCollider>, Without<ScanCollider>),
    >,
    q_pushables: Query<
        (Entity, &Transform, Has<MoveToLocation>, &Children),
        (With<CanMove>, With<Pushable>),
    >,
) {
    let grid_tile_size = s_grid_tile_size.into_inner();

    for (entity, intent, duration, can_push, movement_flag_option, transform, children) in
        q_entities_with_intent.iter()
    {
        let step = (intent.0 * grid_tile_size.0.as_vec2()).extend(0.);

        compute_intent(
            &mut commands,
            entity,
            transform,
            intent,
            duration,
            step,
            children,
            can_push,
            movement_flag_option,
            &q_colliders,
            &q_pushables,
            false,
        );
    }
}
