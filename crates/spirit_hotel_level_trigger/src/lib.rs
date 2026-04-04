use crate::systems::prelude::*;
use bevy::prelude::*;

mod components;
pub mod systems;

pub struct SpiritHotelLevelTriggerPlugin;

impl Plugin for SpiritHotelLevelTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_object_is_trigger)
            .add_observer(load_level_from_activated_trigger);
    }
}
