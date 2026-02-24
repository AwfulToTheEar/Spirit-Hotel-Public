pub mod systems;

use bevy::prelude::*;
use crate::systems::prelude::handle_inventory_item_tiled_object_add_visuals;

pub struct SpiritHotelLevelInventoryPlugin;

impl Plugin for SpiritHotelLevelInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_inventory_item_tiled_object_add_visuals);
    }
}
