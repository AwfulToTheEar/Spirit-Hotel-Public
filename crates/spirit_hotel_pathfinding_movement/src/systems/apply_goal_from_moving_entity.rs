use bevy::prelude::*;
use bevy_northstar::{
    prelude::{AgentOfGrid, AgentPos, DebugPath, Pathfind},
};
use bevy_northstar::prelude::NextPos;
use spirit_hotel_movement::components::prelude::{CanMove};
use spirit_hotel_pathfinding::components::prelude::PathNode;
use crate::components::prelude::{Follow, WithoutPathfinding};

pub fn apply_goal_from_moving_entity(
    mut commands: Commands,
    mut q_movables: Query<
        (Entity, &mut Follow, &AgentOfGrid, Has<WithoutPathfinding>),
        (
            With<CanMove>,
            With<AgentPos>,
        ),
    >,
    q_moving_target: Query<(&AgentPos, &AgentOfGrid), (With<CanMove>, Changed<AgentPos>, Without<PathNode>)>,
) {
    for (entity, goal, agent_of_grid, moveable_without_pathfinding) in q_movables.iter_mut() {
        let Ok((goal_pos, other_agent_of_grid)) = q_moving_target.get(goal.0) else {
            continue;
        };

        if agent_of_grid.0 != other_agent_of_grid.0 {
            continue;
        }

        if moveable_without_pathfinding {
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
