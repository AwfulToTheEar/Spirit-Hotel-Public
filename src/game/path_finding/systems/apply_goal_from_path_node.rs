use bevy::prelude::*;
use bevy_northstar::{
    path::Path,
    prelude::{AgentOfGrid, AgentPos, DebugPath, Pathfind},
};

use crate::game::{
    movement::components::prelude::{CanMove, MoveToLocation},
    path_finding::components::prelude::{FollowPath, PathNode},
};

pub fn apply_goal_from_path_node(
    mut commands: Commands,
    mut q_movables: Query<
        (Entity, &mut FollowPath, &AgentOfGrid),
        (
            With<CanMove>,
            Without<Pathfind>,
            Without<Path>,
            Without<MoveToLocation>,
        ),
    >,
    q_path_nodes: Query<(&AgentPos, &PathNode, &AgentOfGrid), Without<CanMove>>,
) {
    for (entity, mut goal, agent_of_grid) in q_movables.iter_mut() {
        let Some(goal_entity) = goal.0 else {
            continue;
        };

        let Ok((goal_pos, next, path_of_grid)) = q_path_nodes.get(goal_entity) else {
            continue;
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
            DebugPath::default(),
        ));
    }
}
