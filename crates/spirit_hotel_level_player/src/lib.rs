use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::system_sets::prelude::LevelSystems;

pub mod systems;

pub struct SpiritHotelLevelPlayerPlugin;

impl Plugin for SpiritHotelLevelPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            reparent_player_to_world
                .in_set(LevelSystems)
                .in_set(GameplaySystems),
        );
    }
}