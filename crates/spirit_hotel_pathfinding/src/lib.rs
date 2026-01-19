use bevy_northstar::{plugin::NorthstarPlugin, prelude::CardinalNeighborhood};

use crate::system_sets::prelude::PathfindingSystems;
use crate::{
    components::prelude::{InWorldGrid, PathNode},
    systems::prelude::*,
};
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::system_sets::prelude::LevelSystems;

pub mod components;
pub mod system_sets;
pub mod systems;

pub struct SpiritHotelPathfindingPlugin;

impl Plugin for SpiritHotelPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NorthstarPlugin::<CardinalNeighborhood>::default())
            .register_type::<PathNode>()
            .register_type::<InWorldGrid>()
            .configure_sets(PreUpdate, PathfindingSystems.after(LevelSystems))
            .configure_sets(Update, PathfindingSystems.after(LevelSystems))
            .configure_sets(PostUpdate, PathfindingSystems.after(LevelSystems))
            .add_systems(
                PreUpdate,
                (build_world_path_finding_grid, mark_agents_in_world_grid)
                    .chain()
                    .in_set(GameplaySystems)
                    .in_set(PathfindingSystems)
            )
            .add_systems(
                PostUpdate,
                clear_map_for_recompution_when_collider_disabled
                    .in_set(GameplaySystems)
                    .in_set(PathfindingSystems)
            );
    }
}
