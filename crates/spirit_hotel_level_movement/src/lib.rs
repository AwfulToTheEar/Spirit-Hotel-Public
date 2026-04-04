use crate::systems::prelude::*;
use bevy::prelude::*;

pub mod systems;

pub struct SpiritHotelLevelMovementPlugin;

impl Plugin for SpiritHotelLevelMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_object_can_move);
    }
}
