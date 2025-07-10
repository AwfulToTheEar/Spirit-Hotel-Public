use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use bevy_northstar::{grid::Grid, nav::Nav, prelude::CardinalNeighborhood};

use crate::game::path_finding::components::prelude::GridCollisionCalculated;

//very inefficient
pub fn clear_map_for_recompution_when_collider_disabled(
    mut commands: Commands,
    mut q_grids: Query<&mut Grid<CardinalNeighborhood>>,
    q_disabled_colliders: Query<Entity, (Added<Disabled>, With<GridCollisionCalculated>)>,
    q_colliders: Query<Entity, With<GridCollisionCalculated>>,
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
