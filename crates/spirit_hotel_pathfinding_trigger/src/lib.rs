use crate::systems::prelude::compute_grid_collision;
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::system_sets::prelude::LevelSystems;
use spirit_hotel_pathfinding::system_sets::prelude::PathfindingSystems;

pub mod systems;

pub struct SpiritHotelPathfindingTriggerPlugin;

impl Plugin for SpiritHotelPathfindingTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            compute_grid_collision
                .in_set(GameplaySystems)
                .after(PathfindingSystems),
        );
    }
}
