use bevy::prelude::*;
use crate::systems::prelude::*;

pub mod systems;

pub struct SpiritHotelPlayerMovementPlugin;

impl Plugin for SpiritHotelPlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(record_player_input_horizontal)
            .add_observer(record_player_input_vertical);
    }
}
