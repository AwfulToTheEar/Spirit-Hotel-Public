use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::AppSystems;
use spirit_hotel_core::game::GameplaySystems;

pub mod systems;

pub struct SpiritHotelCutsceneTriggerPlugin;

impl Plugin for SpiritHotelCutsceneTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            start_cutscene_from_activated_trigger
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems),
        );
    }
}
