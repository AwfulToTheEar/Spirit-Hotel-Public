use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::system_sets::prelude::LevelSystems;
use crate::systems::prelude::*;

pub mod systems;

pub struct SpiritHotelLevelDrownedChildPlugin;

impl Plugin for SpiritHotelLevelDrownedChildPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(handle_world_init_breath_timer);
    }
}
