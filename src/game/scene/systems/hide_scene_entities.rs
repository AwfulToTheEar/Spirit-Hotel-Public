use bevy::prelude::*;

use crate::game::scene::{
    components::prelude::{GameSceneObject, HideInGameScene, ShowInGameScene},
    resources::prelude::CurrentGameScenes,
};

pub fn hide_scene_entities(
    current_scenes: ResMut<CurrentGameScenes>,
    mut q_scenes: Query<
        (
            Option<&HideInGameScene>,
            Option<&ShowInGameScene>,
            &mut Visibility,
        ),
        With<GameSceneObject>,
    >,
) {
    for (hide_in_scene, show_in_scene, mut visibility) in q_scenes.iter_mut() {
        if (hide_in_scene.is_none() || !current_scenes.0.contains(&hide_in_scene.unwrap().0))
            && (show_in_scene.is_none() || current_scenes.0.contains(&show_in_scene.unwrap().0))
        {
            continue;
        }

        *visibility = Visibility::Hidden;
    }
}
