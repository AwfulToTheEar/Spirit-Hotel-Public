use avian2d::{collision::collider::contact_query, prelude::{Collider, RigidBody}};
use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use bevy_ecs_tiled::{prelude::{TiledMap, TiledWorld, TiledWorldMarker, TilemapAnchor}, world::TiledWorldHandle};
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType};
use bevy_northstar::prelude::*;

use crate::{game::{level::{LevelSystems}, movement::{CanMove, MoveToLocation, MovementIntent}, trigger::TriggerCollider, GameplaySystems}, AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins(NorthstarPlugin::<CardinalNeighborhood>::default())

        .register_type::<PathNode>()
        .register_type::<FollowPath>()
        .register_type::<InWorldGrid>()

        .add_systems(PreUpdate,(
                build_world_path_finding_grid,
                mark_agents_in_world_grid
            )
                .chain()
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

        .add_systems(PostUpdate,(
                clear_map_for_recompution_when_collider_disabled,
                compute_grid_collision
            )
                .chain()
                .in_set(GameplaySystems)
                .after(LevelSystems)
        );
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct InWorldGrid;

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
pub struct GridTileSize(pub UVec2);

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

    let tile_size = UVec2::new(first_tiled_map.map.tile_width, first_tiled_map.map.tile_height);

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
            .build()
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
        &TilemapSize::new(grid_size.x as u32, grid_size.y as u32),
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
                Transform::from_translation(offset.extend(0.0))
            ),
            (
                Collider::rectangle((tile_size.x - 2) as f32, (tile_size.y - 2) as f32),
                ScanCollider
            )
        ]
    ));
}

pub fn mark_agents_in_world_grid(
    mut commands: Commands,
    s_world_grid: Single<Entity, (With<TiledWorldMarker>, With<Grid<CardinalNeighborhood>>)>,
    q_agents: Query<Entity, (With<InWorldGrid>, Without<AgentOfGrid>)>
) {
    let world_grid = s_world_grid.into_inner();

    for agent in q_agents.iter() {
        commands.entity(agent).insert(AgentOfGrid(world_grid));
    }
}

fn compute_grid_collision(
    mut commands: Commands,
    mut q_grids: Query<(Entity, &mut Grid<CardinalNeighborhood>, &GridTileSize, &TilemapAnchor)>,
    q_scan_colliders: Query<(&Collider, &ChildOf), With<ScanCollider>>,
    q_colliders: Query<(Entity, &GlobalTransform, &Collider, &RigidBody), (Without<GridCollisionCalculated>, Without<TriggerCollider>)>,
) {
    if q_colliders.is_empty() {
        return;
    }

    for (entity, mut grid, tile_size, anchor) in q_grids.iter_mut() {
        let offset = anchor.as_offset(
            &TilemapSize::new(grid.width(), grid.height()),
            &TilemapGridSize::new(tile_size.0.x as f32, tile_size.0.y as f32),
            &TilemapTileSize::new(tile_size.0.x as f32, tile_size.0.y as f32),
            &TilemapType::Square,
        );

        let Some((collider, _)) = q_scan_colliders
            .iter()
            .filter(|(_, child_of)| child_of.0 == entity)
            .next()
        else {
            continue;
        };

        for (entity, other_collider_transform, other_collider, rigid_body) in q_colliders.iter() {
            if *rigid_body != RigidBody::Static {
                continue;
            }

            for x in 0..grid.width() {
                for y in 0..grid.height() {
                    let collider_translation = offset + (tile_size.0 * UVec2::new(x, y)).as_vec2();

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

                    grid.set_nav(UVec3::new(x, y, 0), Nav::Impassable);
                }
            }

            commands.entity(entity).insert(GridCollisionCalculated);
        }

        grid.build();
    }
}

//very inefficient
fn clear_map_for_recompution_when_collider_disabled(
    mut commands: Commands,
    mut q_grids: Query<&mut Grid<CardinalNeighborhood>>,
    q_disabled_colliders: Query<Entity, (Added<Disabled>, With<GridCollisionCalculated>)>,
    q_colliders: Query<Entity, With<GridCollisionCalculated>>
) {
    if q_disabled_colliders.is_empty() {
        return;
    }

    for entity in q_disabled_colliders.iter() {
        commands.entity(entity).remove::<GridCollisionCalculated>();
    }

    for entity in q_colliders.iter() {
        commands.entity(entity).remove::<GridCollisionCalculated>();
    }

    for mut grid in q_grids.iter_mut() {
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                grid.set_nav(UVec3::new(x, y, 0), Nav::Passable(1));
            }
        }

        grid.build()
    }
}

pub fn set_grid_pos_from_transform(
    mut commands: Commands,
    q_grids: Query<(&Grid<CardinalNeighborhood>, &GridTileSize, &TilemapAnchor)>,
    mut q_grid_items: Query<(Entity, &GlobalTransform, &AgentOfGrid, Option<&mut AgentPos>), (Changed<Transform>, Or<(With<FollowPath>, With<PathNode>)>)>,
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

        let grid_pos_vec = (transform.translation().truncate() - offset + (tile_size.0 / 2).as_vec2()) / tile_size.0.as_vec2();
        let grid_pos_uvec = UVec3::new(grid_pos_vec.x.round() as u32, grid_pos_vec.y.round() as u32, 0);

        let Some(mut grid_pos) = grid_pos_option else {
            commands.entity(entity).insert(
                AgentPos(grid_pos_uvec)
            );

            continue;
        };

        grid_pos.0 = grid_pos_uvec
    }
}

pub fn apply_goal_from_path_node(
    mut commands: Commands,
    mut q_movables: Query<(Entity, &mut FollowPath, &AgentOfGrid), (With<CanMove>, Without<Pathfind>, Without<Path>, Without<MoveToLocation>)>,
    q_path_nodes: Query<(&AgentPos, &PathNode, &AgentOfGrid), Without<CanMove>>
) {
    for (entity,  mut goal, agent_of_grid) in q_movables.iter_mut() {
        let Some(goal_entity) = goal.0 else {
            continue;
        };

        let Ok((goal_pos, next, path_of_grid)) = q_path_nodes.get(goal_entity) else {
            continue
        };

        if agent_of_grid.0 != path_of_grid.0 {
            continue;
        }

        goal.0 = next.0;

        commands.entity(entity).insert((
            Pathfind {
                goal: goal_pos.0,
                ..default()
            },
            DebugPath::default()
        ));
    }
}

pub fn apply_intent_from_next_pos(
    mut commands: Commands,
    q_next_positions: Query<(Entity, &AgentPos, &NextPos), (With<AgentOfGrid>, Without<MovementIntent>, Without<MoveToLocation>, Without<NextPosToBeRemoved>)>
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
    q_next_positions: Query<(Entity, &AgentPos, &NextPos), (With<AgentOfGrid>, Without<MovementIntent>, Without<MoveToLocation>, With<NextPosToBeRemoved>)>
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
