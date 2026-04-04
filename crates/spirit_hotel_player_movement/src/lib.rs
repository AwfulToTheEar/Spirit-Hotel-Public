use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_core::{AppSystems, PausableSystems};

pub mod systems;

pub struct SpiritHotelPlayerMovementPlugin;

impl Plugin for SpiritHotelPlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(record_player_input_horizontal)
            .add_observer(record_player_input_vertical)
            .add_systems(
                Update,
                check_player_input
                    .in_set(AppSystems::Update)
                    .in_set(GameplaySystems)
                    .in_set(PausableSystems),
            );
    }
}
