use bevy::{ecs::entity_disabling::Disabled, prelude::*};

use crate::game::{level::components::prelude::DisabledByVisibility, scene::{components::prelude::{GameSceneObject, HideInGameScene, ShowInGameScene}, resources::prelude::CurrentGameScenes}};

pub fn show_scene_entities(
    current_scenes: ResMut<CurrentGameScenes>,
    mut q_scenes: Query<
        (
            Option<&HideInGameScene>,
            Option<&ShowInGameScene>,
            &mut Visibility,
        ),
        (
            With<Disabled>,
            With<DisabledByVisibility>,
            With<GameSceneObject>,
        ),
    >,
) {
    for (hide_in_scene, show_in_scene, mut visibility) in q_scenes.iter_mut() {
        if (hide_in_scene.is_some() && current_scenes.0.contains(&hide_in_scene.unwrap().0))
            || (show_in_scene.is_some() && !current_scenes.0.contains(&show_in_scene.unwrap().0))
        {
            continue;
        }

        *visibility = Visibility::Inherited
    }
}
