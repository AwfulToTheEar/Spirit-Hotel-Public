use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TilemapAnchor;
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType};
use bevy_northstar::{
    grid::Grid,
    prelude::{AgentOfGrid, AgentPos, CardinalNeighborhood},
};

use crate::game::path_finding::components::prelude::{FollowPath, GridTileSize, PathNode};

pub fn set_grid_pos_from_transform(
    mut commands: Commands,
    q_grids: Query<(&Grid<CardinalNeighborhood>, &GridTileSize, &TilemapAnchor)>,
    mut q_grid_items: Query<
        (
            Entity,
            &GlobalTransform,
            &AgentOfGrid,
            Option<&mut AgentPos>,
        ),
        (Changed<Transform>, Or<(With<FollowPath>, With<PathNode>)>),
    >,
) {
    for (entity, transform, agent_of_grid, grid_pos_option) in q_grid_items.iter_mut() {
        let Ok((grid, tile_size, anchor)) = q_grids.get(agent_of_grid.0) else {
            continue;
        };

        let offset = anchor.as_offset(
            &TilemapSize::new(grid.width(), grid.height()),
            &TilemapGridSize::new(tile_size.0.x as f32, tile_size.0.y as f32),
            &TilemapTileSize::new(tile_size.0.x as f32, tile_size.0.y as f32),
            &TilemapType::Square,
        );

        let grid_pos_vec = (transform.translation().truncate() - offset
            + (tile_size.0 / 2).as_vec2())
            / tile_size.0.as_vec2();
        let grid_pos_uvec = UVec3::new(
            grid_pos_vec.x.round() as u32,
            grid_pos_vec.y.round() as u32,
            0,
        );

        let Some(mut grid_pos) = grid_pos_option else {
            commands.entity(entity).insert(AgentPos(grid_pos_uvec));

            continue;
        };

        grid_pos.0 = grid_pos_uvec
    }
}
