use bevy::prelude::*;
use bevy_northstar::{
    path::Path,
    prelude::{AgentOfGrid, AgentPos, DebugPath, Pathfind},
};
use bevy_northstar::prelude::NextPos;
use spirit_hotel_movement::components::prelude::{CanMove, MoveToLocation, MovementIntent};
use spirit_hotel_pathfinding::components::prelude::PathNode;
use crate::components::prelude::{Follow, WithoutPathfinding};

pub fn apply_goal_from_path_node(
    mut commands: Commands,
    mut q_movables: Query<
        (Entity, &mut Follow, &AgentOfGrid, Has<WithoutPathfinding>),
        (
            With<CanMove>,
            With<AgentPos>,
            Without<Pathfind>,
            Without<Path>,
            Without<NextPos>,
            Without<MovementIntent>,
            Without<MoveToLocation>,
        ),
    >,
    q_path_nodes: Query<(&AgentPos, &PathNode, &AgentOfGrid, Has<WithoutPathfinding>), Without<CanMove>>,
) {
    for (entity, mut goal, agent_of_grid, moveable_without_pathfinding) in q_movables.iter_mut() {
        let Ok((goal_pos, next, path_of_grid, path_node_without_pathfinding)) = q_path_nodes.get(goal.0) else {
            continue;
        };

        if agent_of_grid.0 != path_of_grid.0 {
            continue;
        }

        if let Some(next_entity) = next.0 {
            goal.0 = next_entity;
        } else {
            commands.entity(entity).remove::<Follow>();
        }

        if moveable_without_pathfinding || path_node_without_pathfinding {
            commands.entity(entity).insert(
                NextPos(goal_pos.0)
            );

            continue;
        }

        commands.entity(entity).insert((
            Pathfind {
                goal: goal_pos.0,
                ..default()
            },
            DebugPath::default(),
        ));
    }
}
