use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::system_sets::prelude::LevelSystems;

pub mod systems;

pub struct SpiritHotelLevelMovementPlugin;

impl Plugin for SpiritHotelLevelMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_object_can_move);
    }
}