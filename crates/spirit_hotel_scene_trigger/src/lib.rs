use bevy::prelude::*;

use crate::systems::prelude::change_scene_from_activated_trigger;
use spirit_hotel_core::{AppSystems, game::GameplaySystems};

pub mod systems;

pub struct SpiritHotelSceneTriggerPlugin;

impl Plugin for SpiritHotelSceneTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(change_scene_from_activated_trigger);
    }
}
