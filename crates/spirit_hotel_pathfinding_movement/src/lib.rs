use bevy::prelude::*;
use spirit_hotel_pathfinding::northstar::{plugin::NorthstarPlugin, prelude::CardinalNeighborhood};

use spirit_hotel_core::{AppSystems, PausableSystems};
use spirit_hotel_core::game::GameplaySystems;
use crate::components::prelude::FollowPath;
use crate::system_sets::prelude::PathfindingMovementSystems;
use crate::systems::prelude::{apply_goal_from_path_node, apply_intent_from_next_pos, remove_next_pos, set_grid_pos_from_transform};

pub mod components;
pub mod systems;
pub mod system_sets;

pub struct SpiritHotelPathfindingMovementPlugin;

impl Plugin for SpiritHotelPathfindingMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FollowPath>()
            .configure_sets(PreUpdate, PathfindingMovementSystems)
            .configure_sets(Update, PathfindingMovementSystems)
            .configure_sets(PostUpdate, PathfindingMovementSystems)
            .add_systems(
                Update,
                (
                    set_grid_pos_from_transform,
                    apply_goal_from_path_node,
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
