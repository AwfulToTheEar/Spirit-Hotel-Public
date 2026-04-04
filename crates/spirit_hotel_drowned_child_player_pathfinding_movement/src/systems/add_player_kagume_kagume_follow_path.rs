use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledWorld;
use bevy_northstar::grid::Grid;
use bevy_northstar::neighbor::CardinalNeighborhood;
use bevy_northstar::prelude::AgentOfGrid;
use spirit_hotel_drowned_child::components::prelude::{KagumeKagume, KagumeKagumeStartPathNode};
use spirit_hotel_pathfinding_movement::components::prelude::Follow;
use spirit_hotel_player::components::prelude::Player;

pub fn add_player_kagume_kagume_follow_path(
    _: On<Add, KagumeKagume>,
    mut commands: Commands,
    s_player: Single<Entity, With<Player>>,
    s_kagume_kagume_start_path_node: Single<Entity, With<KagumeKagumeStartPathNode>>,
    s_world_grid: Single<Entity, (With<TiledWorld>, With<Grid<CardinalNeighborhood>>)>,
) {
    let entity = s_player.into_inner();
    let path_node_entity = s_kagume_kagume_start_path_node.into_inner();
    let world_grid_entity = s_world_grid.into_inner();

    commands
        .entity(entity)
        .insert((Follow(path_node_entity), AgentOfGrid(world_grid_entity)));
}
