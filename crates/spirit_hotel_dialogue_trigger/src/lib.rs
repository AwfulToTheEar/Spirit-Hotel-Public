use crate::systems::prelude::start_conversation_from_activated_trigger;
use bevy::prelude::*;

pub mod systems;

pub struct SpiritHotelDialogueTriggerPlugin;

impl Plugin for SpiritHotelDialogueTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(start_conversation_from_activated_trigger);
    }
}
