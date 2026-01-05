use bevy::prelude::*;

use crate::resources::prelude::{CurrentGameScenes, StartingGameScene};

pub fn set_starting_scene(
    mut commands: Commands,
    starting_game_scene: Res<StartingGameScene>,
    mut current_game_scenes: ResMut<CurrentGameScenes>,
) {
    current_game_scenes.0.clear();
    current_game_scenes.0.insert(starting_game_scene.0.clone());

    commands.remove_resource::<StartingGameScene>();
}
