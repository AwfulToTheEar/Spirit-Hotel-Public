use crate::systems::prelude::*;
use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct SpiritHotelCutsceneTriggerPlugin;

impl Plugin for SpiritHotelCutsceneTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(start_cutscene_from_activated_trigger);
    }
}
