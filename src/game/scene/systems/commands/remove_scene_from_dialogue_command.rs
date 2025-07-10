use bevy::prelude::*;

use crate::game::scene::resources::prelude::CurrentGameScenes;

pub fn remove_scene_from_dialogue_command(
    In(scene): In<String>,
    mut current_scenes: ResMut<CurrentGameScenes>,
) {
    current_scenes.0.remove(&scene);
}
