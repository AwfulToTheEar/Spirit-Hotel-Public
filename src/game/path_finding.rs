use std::ops::DerefMut;

use avian2d::{collision::collider::contact_query, prelude::{Collider, ColliderDisabled, RigidBody}};
use bevy::prelude::*;
use bevy_ecs_tiled::{map::TiledMapHandle, prelude::{TiledMap, TiledMapCreated, TiledMapTileLayer, TiledWorld, TilemapAnchor}, world::TiledWorldHandle};
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_northstar::prelude::*;

use crate::{game::{level::InsideMap, movement::{CanMove, MoveToLocation}, trigger::TriggerCollider, GameplaySystems}, AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins(NorthstarPlugin::<CardinalNeighborhood>::default())
        .add_plugins(FilterQueryInspectorPlugin::<With<FollowPath>>::default())

        .register_type::<PathNode>()
        .register_type::<FollowPath>()

        .add_systems(Update,
            (
                build_world_path_finding_grid,
                compute_grid_collision,
                (
                    set_grid_pos_from_transform,
                    apply_goal_from_path_node
                )
                    .in_set(PausableSystems)
            )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
        );

}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(Transform)]
pub struct PathNode(pub Option<Entity>);

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(Transform, CanMove)]
pub struct FollowPath(pub Option<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GridTileSize(pub Vec2);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GridCollisionCalculated;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScanCollider;

pub fn build_world_path_finding_grid(
    mut commands: Commands,
    tiled_worlds: Res<Assets<TiledWorld>>,
    tiled_maps: Res<Assets<TiledMap>>,
    s_world: Single<(Entity, &TiledWorldHandle, &TilemapAnchor), Without<Grid<CardinalNeighborhood>>>,
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

    let tile_size = Vec2::new(first_tiled_map.map.tile_width as f32, first_tiled_map.map.tile_height as f32);

    let grid_size = tiled_world.rect.max / tile_size as Vec2;

    let mut chunk_size = 1;
    for count in 0..19 {
        let grid_size_scale = (grid_size.x * grid_size.y) as u32;
        let divider = 20 - count as u32;

        if grid_size_scale % divider.pow(2) != 0 || grid_size_scale / divider.pow(2) == 1 {
            continue;
        }

        chunk_size = divider;
        break;
    }

    let mut grid = Grid::<CardinalNeighborhood>::new(&GridSettings {
        width: grid_size.x as u32,
        height: grid_size.y as u32,
        depth: 1,
        chunk_size: chunk_size,
        chunk_depth: 1,
        chunk_ordinal: false,
        default_cost: 1,
        default_wall: true,
        collision: true,
        avoidance_distance: 4,
    });

    for (rect, _) in tiled_world.maps.iter() {
        let min = rect.min / tile_size;
        let max = rect.max / tile_size;

        for x in (min.x as u32)..(max.x as u32) {
            for y in (min.y as u32)..(max.y as u32) {
                grid.set_point(UVec3::new(x, y, 0), Point::new(1, false));
            }
        }

    }

    grid.build();

    let offset = anchor.as_offset(
        &TilemapSize::new(grid_size.x as u32, grid_size.y as u32),
        &TilemapGridSize::new(tile_size.x, tile_size.y),
        &TilemapTileSize::new(tile_size.x, tile_size.y),
        &TilemapType::Square,
    );

    commands.entity(entity).insert((
        grid,
        GridTileSize(tile_size),
        children![
            (
                DebugMap {
                    tile_width: tile_size.x as u32,
                    tile_height: tile_size.y as u32,
                    map_type: DebugMapType::Square,
                    draw_chunks: true,
                    draw_points: true,
                    draw_entrances: false,
                    draw_cached_paths: false,
                },
                Transform::from_translation(offset.extend(0.0))
            ),
            (
                Collider::rectangle(tile_size.x - 2., tile_size.y - 2.),
                ScanCollider
            )
        ]
    ));
}

fn compute_grid_collision(
    mut commands: Commands,
    mut s_grid: Single<(&mut Grid<CardinalNeighborhood>, &GridTileSize, &TilemapAnchor)>,
    s_scan_collider: Single<&Collider, With<ScanCollider>>,
    q_colliders: Query<(Entity, &GlobalTransform, &Collider, &RigidBody), (Without<GridCollisionCalculated>, Without<TriggerCollider>, Without<ColliderDisabled>)>,
) {
    if q_colliders.iter().count() == 0 {
        return;
    }

    let (grid, tile_size, anchor) = s_grid.deref_mut();

    let offset = anchor.as_offset(
        &TilemapSize::new(grid.width(), grid.height()),
        &TilemapGridSize::new(tile_size.0.x, tile_size.0.y),
        &TilemapTileSize::new(tile_size.0.x, tile_size.0.y),
        &TilemapType::Square,
    );

    let collider = s_scan_collider.into_inner();

    for (entity, other_collider_transform, other_collider, rigid_body) in q_colliders.iter() {
        if *rigid_body != RigidBody::Static {
            continue;
        }

        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let collider_translation = offset + tile_size.0 * Vec2::new(x as f32, y as f32);

                let Ok(result) = contact_query::intersection_test(
                    collider,
                    collider_translation,
                    Quat::default(),
                    other_collider,
                    other_collider_transform.translation().truncate(),
                    other_collider_transform.rotation()
                ) else {
                    continue;
                };

                if !result {
                    continue;
                }

                grid.set_point(UVec3::new(x, y, 0), Point::new(0, true));
            }
        }

        commands.entity(entity).insert(GridCollisionCalculated);
    }

    grid.build();

}

pub fn set_grid_pos_from_transform(
    mut commands: Commands,
    mut s_grid: Single<(&mut Grid<CardinalNeighborhood>, &GridTileSize, &TilemapAnchor)>,
    mut q_movables: Query<(Entity, &GlobalTransform, Option<&mut GridPos>), (Changed<Transform>, With<FollowPath>, With<CanMove>)>,
) {
    let (grid, tile_size, anchor) = s_grid.deref_mut();

    let offset = anchor.as_offset(
        &TilemapSize::new(grid.width(), grid.height()),
        &TilemapGridSize::new(tile_size.0.x, tile_size.0.y),
        &TilemapTileSize::new(tile_size.0.x, tile_size.0.y),
        &TilemapType::Square,
    );

    for (entity, transform, grid_pos_option) in q_movables.iter_mut() {
        let grid_pos_vec = ((transform.translation().truncate() - Vec2::new(offset.x + tile_size.0.x, -(offset.y + tile_size.0.y))) / tile_size.0).as_uvec2().extend(0);

        info!("real pos: {}, offset: {}, world pos: {}, grid pos: {}", transform.translation().truncate(), Vec2::new(offset.x + tile_size.0.x, -(offset.y + tile_size.0.y)), (transform.translation().truncate() - Vec2::new(offset.x + tile_size.0.x, -(offset.y + tile_size.0.y))), grid_pos_vec);

        let Some(mut grid_pos) = grid_pos_option else {
            commands.entity(entity).insert(
                GridPos(grid_pos_vec)
            );

            continue;
        };

        grid_pos.0 = grid_pos_vec
    }
}

pub fn apply_goal_from_path_node(
    mut commands: Commands,
    s_tile_size: Single<&GridTileSize>,
    mut q_movables: Query<(Entity, &mut FollowPath), (With<CanMove>, Without<Pathfind>, Without<Path>, Without<MoveToLocation>)>,
    q_path_nodes: Query<(&Transform, &PathNode), Without<CanMove>>
) {
    let tile_size = s_tile_size.into_inner().0;

    for (entity,  mut goal) in q_movables.iter_mut() {
        let Some(goal_entity) = goal.0 else {
            continue;
        };

        let Ok((goal_transform, next)) = q_path_nodes.get(goal_entity) else {
            continue
        };

        goal.0 = next.0;

        commands.entity(entity).insert((
            Pathfind {
                goal: (goal_transform.translation.truncate() / tile_size).as_uvec2().extend(0),
                ..default()
            }
        ));
    }
}

pub fn apply_intent_from_next_pos(

) {

}
