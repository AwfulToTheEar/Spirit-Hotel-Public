use spirit_hotel_core::AppSystems;
use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::system_sets::prelude::LevelSystems;

pub mod systems;

pub struct SpiritHotelLevelScenePlugin;

impl Plugin for SpiritHotelLevelScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                show_scene_entities
            )
                .in_set(LevelSystems)
                .in_set(GameplaySystems)
                .in_set(AppSystems::Update),
        );
    }
}