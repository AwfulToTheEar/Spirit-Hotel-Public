use bevy::prelude::*;
use spirit_hotel_core::{AppSystems, game::GameplaySystems, screens::Screen};
use crate::systems::prelude::start_conversation_from_activated_trigger;

pub mod systems;

pub struct SpiritHotelDialogueTriggerPlugin;

impl Plugin for SpiritHotelDialogueTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            start_conversation_from_activated_trigger
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems),
        );
    }
}
