use avian2d::prelude::Collider;
use bevy::prelude::*;
use bevy_northstar::prelude::{AgentOfGrid, CardinalNeighborhood, Grid};
use spirit_hotel_movement::components::prelude::{CanMove, CanPush, CheckMovementIntent, MoveToLocation, MovementDuration, MovementFlag, MovementIntent, Pushable, UpdateMovementIntent};
use spirit_hotel_pathfinding::components::prelude::{GridTileSize, ScanCollider};
use spirit_hotel_pathfinding_movement::components::prelude::Follow;
use spirit_hotel_trigger::components::prelude::TriggerCollider;
use crate::utils::prelude::compute_intent;

//TODO: support multiple grids
pub fn apply_intent(
    mut commands: Commands,
    q_grid_tile_sizes: Query<&GridTileSize, With<Grid<CardinalNeighborhood>>>,
    q_entities_with_intent: Query<
        (
            Entity,
            Option<&Follow>,
            &AgentOfGrid,
            &MovementIntent,
            &MovementDuration,
            Has<CanPush>,
            Option<&MovementFlag>,
            &Transform,
            &Children,
        ),
        (With<CanMove>, Without<MoveToLocation>, Without<CheckMovementIntent>, Without<UpdateMovementIntent>),
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
    for (entity, option_follow, agent_of_grid, intent, duration, can_push, movement_flag_option, transform, children) in
        q_entities_with_intent.iter()
    {
        let Ok(grid_tile_size) = q_grid_tile_sizes.get(agent_of_grid.0) else {
            continue;
        };

        let step = (intent.0 * grid_tile_size.0.as_vec2()).extend(0.);

        compute_intent(
            &mut commands,
            entity,
            option_follow,
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
