use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType};
use bevy_northstar::prelude::DebugOffset;
use bevy_northstar::{
    grid::{Grid, GridSettingsBuilder},
    nav::Nav,
    prelude::{CardinalNeighborhood, DebugGridBuilder},
};

use crate::components::prelude::{GridTileSize, ScanCollider};
use avian2d::prelude::Collider;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMapAsset, TiledWorldAsset};
use bevy_ecs_tiled::prelude::{TiledWorld, TilemapAnchor};

pub fn build_world_path_finding_grid(
    mut commands: Commands,
    tiled_worlds: Res<Assets<TiledWorldAsset>>,
    tiled_maps: Res<Assets<TiledMapAsset>>,
    s_world: Single<(Entity, &TiledWorld, &TilemapAnchor), Without<Grid<CardinalNeighborhood>>>,
) {
    let (entity, world_handle, anchor) = s_world.into_inner();

    let Some(tiled_world) = tiled_worlds.get(world_handle.0.id()) else {
        return;
    };

    let Some(first_map) = tiled_world.maps.first() else {
        return;
    };

    let Some(first_tiled_map) = tiled_maps.get(first_map.1.id()) else {
        return;
    };

    let tile_size = UVec2::new(
        first_tiled_map.map.tile_width,
        first_tiled_map.map.tile_height,
    );

    let grid_size = tiled_world.rect.max.as_uvec2() / tile_size;

    let mut chunk_size = 10;
    for count in 0..10 {
        let grid_size_scale = grid_size.x * grid_size.y;
        let divider = 20 - count as u32;

        if grid_size_scale % divider.pow(2) != 0 || grid_size_scale / divider.pow(2) == 1 {
            continue;
        }

        chunk_size = divider;
        break;
    }

    let mut grid = Grid::<CardinalNeighborhood>::new(
        &GridSettingsBuilder::new_2d(grid_size.x, grid_size.y)
            .chunk_size(chunk_size)
            .default_impassable()
            .enable_collision()
            .avoidance_distance(4)
            .build(),
    );

    for (rect, _) in tiled_world.maps.iter() {
        let min = rect.min.as_uvec2() / tile_size;
        let max = rect.max.as_uvec2() / tile_size;

        for x in min.x..max.x {
            for y in min.y..max.y {
                grid.set_nav(UVec3::new(x, y, 0), Nav::Passable(1));
            }
        }
    }

    grid.build();

    let offset = anchor.as_offset(
        &TilemapSize::new(grid_size.x, grid_size.y),
        &TilemapGridSize::new(tile_size.x as f32, tile_size.y as f32),
        &TilemapTileSize::new(tile_size.x as f32, tile_size.y as f32),
        &TilemapType::Square,
    );

    commands.entity(entity).insert((
        grid,
        GridTileSize(tile_size),
        children![
            (
                DebugGridBuilder::new(tile_size.x, tile_size.y)
                    .enable_chunks()
                    .enable_cells()
                    .build(),
                DebugOffset(offset.extend(0.0)),
                Transform::from_translation(offset.extend(0.0))
            ),
            (
                Collider::rectangle((tile_size.x - 2) as f32, (tile_size.y - 2) as f32),
                ScanCollider
            )
        ],
    ));
}
