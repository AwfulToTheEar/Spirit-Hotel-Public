use crate::game::drowned_child::components::prelude::KagumeKagumeStartPathNode;
use crate::game::path_finding::components::prelude::FollowPath;
use crate::game::player::components::prelude::Player;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledWorld;
use bevy_northstar::grid::Grid;
use bevy_northstar::neighbor::CardinalNeighborhood;
use bevy_northstar::prelude::AgentOfGrid;

pub fn add_player_kagume_kagume_follow_path(
    mut commands: Commands,
    s_player: Single<Entity, With<Player>>,
    s_kagume_kagume_start_path_node: Single<Entity, With<KagumeKagumeStartPathNode>>,
    s_world_grid: Single<Entity, (With<TiledWorld>, With<Grid<CardinalNeighborhood>>)>,
) {
    let entity = s_player.into_inner();
    let path_node_entity = s_kagume_kagume_start_path_node.into_inner();
    let world_grid_entity = s_world_grid.into_inner();

    commands.entity(entity).insert((
        FollowPath(Some(path_node_entity)),
        AgentOfGrid(world_grid_entity),
    ));
}
