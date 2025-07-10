use bevy::prelude::*;
use bevy_northstar::{plugin::NorthstarPlugin, prelude::CardinalNeighborhood};

use crate::{
    AppSystems, PausableSystems,
    game::{
        GameplaySystems,
        level::system_sets::prelude::LevelSystems,
        path_finding::{
            components::prelude::{FollowPath, InWorldGrid, PathNode},
            systems::prelude::*,
        },
    },
};

pub mod components;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.add_plugins(NorthstarPlugin::<CardinalNeighborhood>::default())
        .register_type::<PathNode>()
        .register_type::<FollowPath>()
        .register_type::<InWorldGrid>()
        .add_systems(
            PreUpdate,
            (build_world_path_finding_grid, mark_agents_in_world_grid)
                .chain()
                .in_set(GameplaySystems)
                .after(LevelSystems),
        )
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
                .in_set(GameplaySystems),
        )
        .add_systems(
            PostUpdate,
            (
                clear_map_for_recompution_when_collider_disabled,
                compute_grid_collision,
            )
                .chain()
                .in_set(GameplaySystems)
                .after(LevelSystems),
        );
}
