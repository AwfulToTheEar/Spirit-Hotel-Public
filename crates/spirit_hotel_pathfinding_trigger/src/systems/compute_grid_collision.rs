use avian2d::{
    collision::collider::contact_query,
    prelude::{Collider, RigidBody},
};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TilemapAnchor;
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType};
use bevy_northstar::{grid::Grid, nav::Nav, prelude::CardinalNeighborhood};
use spirit_hotel_pathfinding::components::prelude::{
    GridCollisionCalculated, GridTileSize, ScanCollider,
};
use spirit_hotel_trigger::components::prelude::TriggerCollider;

pub fn compute_grid_collision(
    mut commands: Commands,
    mut q_grids: Query<(
        Entity,
        &mut Grid<CardinalNeighborhood>,
        &GridTileSize,
        &TilemapAnchor,
    )>,
    q_scan_colliders: Query<(&Collider, &ChildOf), With<ScanCollider>>,
    q_colliders: Query<
        (Entity, &GlobalTransform, &Collider, Option<&RigidBody>),
        (
            Without<GridCollisionCalculated>,
            Without<TriggerCollider>,
            Without<ScanCollider>,
        ),
    >,
) {
    let filtered_colliders = q_colliders.iter().filter(|(_, _, _, optional_rigid_body)| {
        optional_rigid_body.is_none() || *optional_rigid_body.unwrap() == RigidBody::Static
    });

    if filtered_colliders.clone().count() == 0 {
        return;
    }

    info!("Computing grid collisions...");

    for (grid_entity, mut grid, tile_size, anchor) in q_grids.iter_mut() {
        let offset = anchor.as_offset(
            &TilemapSize::new(grid.width(), grid.height()),
            &TilemapGridSize::new(tile_size.0.x as f32, tile_size.0.y as f32),
            &TilemapTileSize::new(tile_size.0.x as f32, tile_size.0.y as f32),
            &TilemapType::Square,
        );

        let Some((collider, _)) = q_scan_colliders
            .iter()
            .find(|(_, child_of)| child_of.0 == grid_entity)
        else {
            continue;
        };

        for (collider_entity, other_collider_transform, other_collider, _) in
            filtered_colliders.clone()
        {
            info!("{}", collider_entity);
            for x in 0..grid.width() {
                for y in 0..grid.height() {
                    let collider_translation = offset + (tile_size.0 * UVec2::new(x, y)).as_vec2();

                    let Ok(result) = contact_query::intersection_test(
                        collider,
                        collider_translation,
                        Quat::default(),
                        other_collider,
                        other_collider_transform.translation().truncate(),
                        other_collider_transform.rotation(),
                    ) else {
                        continue;
                    };

                    if !result {
                        continue;
                    }

                    grid.set_nav(UVec3::new(x, y, 0), Nav::Impassable);
                }
            }

            commands
                .entity(collider_entity)
                .insert(GridCollisionCalculated);
        }

        grid.build();
    }
}
