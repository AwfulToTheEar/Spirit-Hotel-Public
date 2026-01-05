use bevy::prelude::*;
use spirit_hotel_scene::resources::prelude::CurrentGameScenes;

pub fn toggle_scene_from_dialogue_command(
    In(scene): In<String>,
    mut current_scenes: ResMut<CurrentGameScenes>,
) {
    if current_scenes.0.contains(&scene) {
        current_scenes.0.remove(&scene);
        return;
    }
    current_scenes.0.insert(scene.clone());
}
