use bevy::{ecs::entity_disabling::Disabled, platform::collections::HashSet, prelude::*};

use crate::{game::{level::DisabledByVisibility, trigger::{IsTrigger, TriggerActivatedEvent}, GameplaySystems}, AppSystems};

pub(super) fn plugin(app: &mut App) {
    app
        .register_type::<ShowInGameScene>()
        .register_type::<HideInGameScene>()
        .register_type::<ChangeGameScene>()
        .register_type::<StartingGameScene>()
        .register_type::<CurrentGameScenes>()

        .init_resource::<CurrentGameScenes>()

        .add_systems(PreUpdate,
            set_starting_scene
                .in_set(GameplaySystems)
                .run_if(resource_added::<StartingGameScene>)
        )
        .add_systems(Update,(
                change_scene_from_activated_trigger,
                hide_scene_entities,
                show_scene_entities
            )
                .in_set(GameplaySystems)
                .in_set(AppSystems::Update)
        );

}

#[derive(Resource, Default, Clone, Reflect)]
#[reflect(Resource, Default)]
pub struct CurrentGameScenes(pub HashSet<String>);

#[derive(Resource, Default, Clone, Reflect)]
#[reflect(Resource, Default)]
pub struct StartingGameScene(pub String);

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct GameSceneObject;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(GameSceneObject)]
pub struct ShowInGameScene(pub String);

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(GameSceneObject)]
pub struct HideInGameScene(pub String);

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct ChangeGameScene {
    pub scene: String,
    pub action: ChangeGameSceneAction
}

#[derive(Default, Debug, Reflect)]
#[reflect(Default)]
pub enum ChangeGameSceneAction {
    #[default]
    Toggle,
    Add,
    Remove
}

pub fn hide_scene_entities(
    current_scenes: ResMut<CurrentGameScenes>,
    mut q_scenes: Query<(Option<&HideInGameScene>, Option<&ShowInGameScene>, &mut Visibility), With<GameSceneObject>>
) {
    for (hide_in_scene, show_in_scene, mut visibility) in q_scenes.iter_mut() {
        if
            (hide_in_scene.is_none() || !current_scenes.0.contains(&hide_in_scene.unwrap().0)) &&
            (show_in_scene.is_none() || current_scenes.0.contains(&show_in_scene.unwrap().0))
        {
            continue;
        }

        *visibility = Visibility::Hidden;
    }
}

pub fn show_scene_entities(
    current_scenes: ResMut<CurrentGameScenes>,
    mut q_scenes: Query<(Option<&HideInGameScene>, Option<&ShowInGameScene>, &mut Visibility), (With<Disabled>, With<DisabledByVisibility>, With<GameSceneObject>)>
) {
    for (hide_in_scene, show_in_scene, mut visibility) in q_scenes.iter_mut() {
        if
            (hide_in_scene.is_some() && current_scenes.0.contains(&hide_in_scene.unwrap().0)) ||
            (show_in_scene.is_some() && !current_scenes.0.contains(&show_in_scene.unwrap().0))
        {
            continue;
        }

        *visibility = Visibility::Inherited
    }
}

pub fn set_starting_scene(
    mut commands: Commands,
    starting_game_scene: Res<StartingGameScene>,
    mut current_game_scenes: ResMut<CurrentGameScenes>
) {
    current_game_scenes.0.clear();
    current_game_scenes.0.insert(starting_game_scene.0.clone());

    commands.remove_resource::<StartingGameScene>();
}

pub fn change_scene_from_activated_trigger(
    mut current_scenes: ResMut<CurrentGameScenes>,
    mut e_trigger_activated: EventReader<TriggerActivatedEvent>,
    q_scene_setters: Query<&ChangeGameScene, With<IsTrigger>>
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
