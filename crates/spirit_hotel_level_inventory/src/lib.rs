pub mod systems;

use crate::systems::prelude::handle_inventory_item_tiled_object_add_visuals;
use bevy::prelude::*;

pub struct SpiritHotelLevelInventoryPlugin;

impl Plugin for SpiritHotelLevelInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_inventory_item_tiled_object_add_visuals);
    }
}
