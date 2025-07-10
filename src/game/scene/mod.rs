use bevy::prelude::*;

use crate::{game::{scene::{components::prelude::{ChangeGameScene, HideInGameScene, ShowInGameScene}, resources::prelude::{CurrentGameScenes, StartingGameScene}, systems::prelude::*}, GameplaySystems}, AppSystems};

pub mod components;
pub mod resources;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.register_type::<ShowInGameScene>()
        .register_type::<HideInGameScene>()
        .register_type::<ChangeGameScene>()
        .register_type::<StartingGameScene>()
        .register_type::<CurrentGameScenes>()
        .init_resource::<CurrentGameScenes>()
        .add_systems(
            PreUpdate,
            set_starting_scene
                .in_set(GameplaySystems)
                .run_if(resource_added::<StartingGameScene>),
        )
        .add_systems(
            Update,
            (
                change_scene_from_activated_trigger,
                hide_scene_entities,
                show_scene_entities,
            )
                .in_set(GameplaySystems)
                .in_set(AppSystems::Update),
        );
}
