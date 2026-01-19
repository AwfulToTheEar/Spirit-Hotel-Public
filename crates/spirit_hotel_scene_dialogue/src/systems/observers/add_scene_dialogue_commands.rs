use bevy::prelude::*;
use bevy_yarnspinner::prelude::DialogueRunner;
use crate::systems::commands::prelude::*;

pub fn add_scene_dialogue_commands(
    event: On<Add, DialogueRunner>,
    mut commands: Commands,
    mut q_dialogue_runners: Query<&DialogueRunner>
) {
    /*let mut dialogue_runner = q_dialogue_runners.get_mut(event.entity).unwrap();

    dialogue_runner
        .commands_mut()
        .add_command(
            "add_game_scene",
            commands.register_system(add_scene_from_dialogue_command),
        )
        .add_command(
            "remove_game_scene",
            commands.register_system(remove_scene_from_dialogue_command),
        )
        .add_command(
            "toggle_game_scene",
            commands.register_system(toggle_scene_from_dialogue_command),
        );*/
}