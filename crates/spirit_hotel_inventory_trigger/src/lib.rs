pub mod components;
pub mod systems;

use crate::systems::prelude::{
    add_item_from_ref_on_activated_trigger, add_item_from_world_on_activated_trigger,
    remove_item_from_ref_on_activated_trigger,
};
use bevy::prelude::*;

pub struct SpiritHotelInventoryTriggerPlugin;

impl Plugin for SpiritHotelInventoryTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(remove_item_from_ref_on_activated_trigger)
            .add_observer(add_item_from_ref_on_activated_trigger)
            .add_observer(add_item_from_world_on_activated_trigger);
    }
}
