use bevy_northstar::{
    grid::Grid,
    prelude::{AgentOfGrid, CardinalNeighborhood},
};

use bevy::prelude::*;
use spirit_hotel_level::tiled::prelude::{TiledWorld};
use crate::components::prelude::InWorldGrid;

pub fn mark_agents_in_world_grid(
    mut commands: Commands,
    s_world_grid: Single<Entity, (With<TiledWorld>, With<Grid<CardinalNeighborhood>>)>,
    q_agents: Query<Entity, (With<InWorldGrid>, Without<AgentOfGrid>)>,
) {
    let world_grid = s_world_grid.into_inner();

    for agent in q_agents.iter() {
        commands.entity(agent).insert(AgentOfGrid(world_grid));
    }
}
