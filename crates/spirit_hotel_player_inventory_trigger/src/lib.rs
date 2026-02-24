mod systems;

use crate::systems::prelude::handle_add_inventory_action_use;
use bevy::prelude::*;

pub struct SpiritHotelPlayerInventoryTriggerPlugin;

impl Plugin for SpiritHotelPlayerInventoryTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_add_inventory_action_use);
    }
}
