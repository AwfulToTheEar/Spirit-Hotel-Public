use bevy::prelude::*;
use spirit_hotel_scene::resources::prelude::CurrentGameScenes;

pub fn add_scene_from_dialogue_command(
    In(scene): In<String>,
    mut current_scenes: ResMut<CurrentGameScenes>,
) {
    current_scenes.0.insert(scene.clone());
}
