pub mod assets;
pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use crate::resources::prelude::InventoryManifest;
use crate::systems::prelude::*;
use bevy::prelude::*;
use leafwing_manifest::plugin::RegisterManifest;

pub struct SpiritHotelInventoryPlugin;

impl Plugin for SpiritHotelInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_manifest::<InventoryManifest>("inventory_items.ron")
            .add_observer(handle_inventory_item_ref_add_inventory_item)
            .add_observer(handle_inventory_item_node_add_visuals)
            .add_observer(handle_inventory_item_selected)
            .add_observer(handle_inventory_section_spawned)
            .add_observer(handle_inventory_menu_spawned)
            .add_observer(handle_add_selected_item_highlight);
    }
}
