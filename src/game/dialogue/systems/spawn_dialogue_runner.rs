use bevy::{ecs::system::SystemState, prelude::*};
use bevy_yarnspinner::prelude::YarnProject;

use crate::{game::scene::systems::commands::prelude::*, screens::Screen};

pub fn spawn_dialogue_runner(
    world: &mut World,
    params: &mut SystemState<(Res<YarnProject>, Commands)>,
) {
    let (project, mut commands) = params.get_mut(world);
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);

    dialogue_runner
        .commands_mut()
        .add_command(
            "add_game_scene",
            world.register_system(add_scene_from_dialogue_command),
        )
        .add_command(
            "remove_game_scene",
            world.register_system(remove_scene_from_dialogue_command),
        )
        .add_command(
            "toggle_game_scene",
            world.register_system(toggle_scene_from_dialogue_command),
        );

    world.spawn((dialogue_runner, StateScoped(Screen::Gameplay)));
}
