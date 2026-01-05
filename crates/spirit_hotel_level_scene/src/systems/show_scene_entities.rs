use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use spirit_hotel_level::components::prelude::DisabledByVisibility;
use spirit_hotel_scene::components::prelude::{GameSceneObject, HideInGameScene, ShowInGameScene};
use spirit_hotel_scene::resources::prelude::CurrentGameScenes;

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
