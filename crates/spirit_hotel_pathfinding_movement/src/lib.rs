use bevy::prelude::*;

use crate::components::prelude::{Follow, WithoutPathfinding};
use crate::system_sets::prelude::PathfindingMovementSystems;
use crate::systems::prelude::{
    apply_goal_from_moving_entity, apply_goal_from_path_node, apply_intent_from_next_pos,
    remove_next_pos, set_grid_pos_from_transform,
};
use spirit_hotel_core::game::{GameplaySystems, teleport};
use spirit_hotel_core::{AppSystems, PausableSystems};

pub mod components;
pub mod system_sets;
pub mod systems;

pub struct SpiritHotelPathfindingMovementPlugin;

impl Plugin for SpiritHotelPathfindingMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Follow>()
            .register_type::<WithoutPathfinding>()
            .configure_sets(PreUpdate, PathfindingMovementSystems)
            .configure_sets(Update, PathfindingMovementSystems)
            .configure_sets(PostUpdate, PathfindingMovementSystems)
            .add_systems(
                PreUpdate,
                set_grid_pos_from_transform
                    .in_set(GameplaySystems)
                    .in_set(PathfindingMovementSystems),
            )
            .add_systems(
                Update,
                (
                    (apply_goal_from_path_node, apply_goal_from_moving_entity),
                    (remove_next_pos, apply_intent_from_next_pos),
                )
                    .chain()
                    .in_set(PausableSystems)
                    .in_set(AppSystems::Update)
                    .in_set(GameplaySystems)
                    .in_set(PathfindingMovementSystems),
            );
    }
}
