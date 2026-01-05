use spirit_hotel_core::AppSystems;
use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::resources::prelude::LevelAssets;
use spirit_hotel_level::system_sets::prelude::LevelSystems;

pub mod systems;

pub struct SpiritHotelLevelTriggerPlugin;

impl Plugin for SpiritHotelLevelTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (handle_object_is_trigger)
                .after(LevelSystems)
                .in_set(GameplaySystems),
        ).add_systems(
            Update,
            (
                load_level_from_activated_trigger.run_if(resource_exists::<LevelAssets>),
            )
                .in_set(LevelSystems)
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems),
        );
    }
}