use bevy::prelude::*;

use crate::components::prelude::{ChangeGameScene, HideInGameScene, ShowInGameScene};
use crate::resources::prelude::{CurrentGameScenes, StartingGameScene};
use crate::system_sets::prelude::SceneSystems;
use crate::systems::prelude::{hide_scene_entities, set_starting_scene};
use spirit_hotel_core::{AppSystems, game::GameplaySystems};

pub mod components;
pub mod resources;
pub mod system_sets;
pub mod systems;

pub struct SpiritHotelScenePlugin;

impl Plugin for SpiritHotelScenePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ShowInGameScene>()
            .register_type::<HideInGameScene>()
            .register_type::<ChangeGameScene>()
            .register_type::<StartingGameScene>()
            .register_type::<CurrentGameScenes>()
            .init_resource::<CurrentGameScenes>()
            .configure_sets(PreUpdate, SceneSystems)
            .configure_sets(Update, SceneSystems)
            .configure_sets(PostUpdate, SceneSystems)
            .add_systems(
                PreUpdate,
                set_starting_scene
                    .in_set(GameplaySystems)
                    .in_set(SceneSystems)
                    .run_if(resource_added::<StartingGameScene>),
            )
            .add_systems(
                Update,
                (hide_scene_entities,)
                    .in_set(GameplaySystems)
                    .in_set(SceneSystems)
                    .in_set(AppSystems::Update),
            );
    }
}
