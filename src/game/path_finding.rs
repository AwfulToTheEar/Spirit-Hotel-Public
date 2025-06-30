use std::ops::DerefMut;

use avian2d::{collision::collider::contact_query, prelude::{Collider, RigidBody}};
use bevy::prelude::*;
use bevy_ecs_tiled::{map::TiledMapHandle, prelude::{TiledMap, TiledMapCreated, TiledMapTileLayer, TiledWorld, TilemapAnchor}, world::TiledWorldHandle};
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType};
use bevy_northstar::prelude::*;

use crate::{game::{level::{InsideMap, LevelSystems}, movement::{remove_move_to_location, CanMove, MoveToLocation, MovementIntent}, trigger::TriggerCollider, GameplaySystems}, AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins(NorthstarPlugin::<CardinalNeighborhood>::default())

        .register_type::<PathNode>()
        .register_type::<FollowPath>()

        .add_systems(PreUpdate,
            build_world_path_finding_grid
                .in_set(GameplaySystems)
                .after(LevelSystems)
        )

        .add_systems(Update,
            (
                set_grid_pos_from_transform,
                apply_goal_from_path_node,
                (
                    remove_next_pos,
                    apply_intent_from_next_pos
                )
            )
                .chain()
                .in_set(PausableSystems)
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
        )

        .add_systems(PostUpdate,
            compute_grid_collision
                .in_set(GameplaySystems)
                .after(LevelSystems)
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

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct NextPosToBeRemoved;

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

    let mut chunk_size = 10;
    for count in 0..10 {
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
    q_colliders: Query<(Entity, &GlobalTransform, &Collider, &RigidBody), (Without<GridCollisionCalculated>, Without<TriggerCollider>)>,
) {
    if q_colliders.is_empty() {
        return;
    }

    info!("computed grid");

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
    mut q_grid_items: Query<(Entity, &GlobalTransform, Option<&mut GridPos>), (Changed<Transform>, Or<(With<FollowPath>, With<PathNode>)>)>,
) {
    let (grid, tile_size, anchor) = s_grid.deref_mut();

    let offset = anchor.as_offset(
        &TilemapSize::new(grid.width(), grid.height()),
        &TilemapGridSize::new(tile_size.0.x, tile_size.0.y),
        &TilemapTileSize::new(tile_size.0.x, tile_size.0.y),
        &TilemapType::Square,
    );

    for (entity, transform, grid_pos_option) in q_grid_items.iter_mut() {
        let grid_pos_vec = (transform.translation().truncate() - offset + tile_size.0 / 2.) / tile_size.0;
        let grid_pos_uvec = UVec3::new(grid_pos_vec.x.round() as u32, grid_pos_vec.y.round() as u32, 0);

        let Some(mut grid_pos) = grid_pos_option else {
            commands.entity(entity).insert(
                GridPos(grid_pos_uvec)
            );

            continue;
        };

        grid_pos.0 = grid_pos_uvec
    }
}

pub fn apply_goal_from_path_node(
    mut commands: Commands,
    mut q_movables: Query<(Entity, &mut FollowPath, &GridPos), (With<GridPos>, With<CanMove>, Without<Pathfind>, Without<Path>, Without<MoveToLocation>)>,
    q_path_nodes: Query<(&GridPos, &PathNode), Without<CanMove>>
) {
    for (entity,  mut goal, start_pos) in q_movables.iter_mut() {
        let Some(goal_entity) = goal.0 else {
            continue;
        };

        let Ok((goal_pos, next)) = q_path_nodes.get(goal_entity) else {
            continue
        };

        goal.0 = next.0;

        commands.entity(entity).insert((
            Pathfind {
                goal: goal_pos.0,
                ..default()
            },
            DebugPath {
                tile_width: 32,
                tile_height: 32,
                map_type: DebugMapType::Square,
                ..default()
            }
        ));
    }
}

pub fn apply_intent_from_next_pos(
    mut commands: Commands,
    q_next_positions: Query<(Entity, &GridPos, &NextPos), (Without<MovementIntent>, Without<MoveToLocation>, Without<NextPosToBeRemoved>)>
) {
    for (entity, grid_pos, next_pos) in q_next_positions.iter() {
        commands.entity(entity).insert((
            MovementIntent((next_pos.0.as_vec3() - grid_pos.0.as_vec3()).truncate().normalize()),
            NextPosToBeRemoved
        ));
    }
}

pub fn remove_next_pos(
    mut commands: Commands,
    q_next_positions: Query<(Entity, &GridPos, &NextPos), (Without<MovementIntent>, Without<MoveToLocation>, With<NextPosToBeRemoved>)>
) {
    for (entity, grid_pos, next_pos) in q_next_positions.iter() {
        if grid_pos.0 != next_pos.0 {
            //TODO: do something when position can't be reached
            warn!("position not reached");
            continue;
        }

        commands.entity(entity).remove::<(
            NextPos,
            NextPosToBeRemoved
        )>();
    }
}
