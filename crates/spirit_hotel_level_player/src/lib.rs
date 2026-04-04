use crate::systems::prelude::*;
use bevy::prelude::*;

pub mod systems;

pub struct SpiritHotelLevelPlayerPlugin;

impl Plugin for SpiritHotelLevelPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(reparent_player_to_world);
    }
}