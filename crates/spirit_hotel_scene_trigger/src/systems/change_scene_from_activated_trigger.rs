use bevy::prelude::*;
use spirit_hotel_scene::components::prelude::{ChangeGameScene, ChangeGameSceneAction};
use spirit_hotel_scene::resources::prelude::CurrentGameScenes;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::messages::prelude::TriggerActivatedMessage;

pub fn change_scene_from_activated_trigger(
    mut current_scenes: ResMut<CurrentGameScenes>,
    mut e_trigger_activated: MessageReader<TriggerActivatedMessage>,
    q_scene_setters: Query<&ChangeGameScene, With<IsTrigger>>,
) {
    for e in e_trigger_activated.read() {
        let Ok(change_game_scene) = q_scene_setters.get(e.trigger) else {
            continue;
        };

        match change_game_scene.action {
            ChangeGameSceneAction::Add => {
                current_scenes.0.insert(change_game_scene.scene.clone());
            }
            ChangeGameSceneAction::Remove => {
                current_scenes.0.remove(&change_game_scene.scene);
            }
            ChangeGameSceneAction::Toggle => {
                if current_scenes.0.contains(&change_game_scene.scene) {
                    current_scenes.0.remove(&change_game_scene.scene);
                    continue;
                }
                current_scenes.0.insert(change_game_scene.scene.clone());
            }
        }
    }
}
