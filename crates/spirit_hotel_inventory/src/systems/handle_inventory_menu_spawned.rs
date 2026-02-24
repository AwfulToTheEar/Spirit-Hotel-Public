use crate::systems::prelude::{handle_render_inventory};
use bevy::prelude::*;
use spirit_hotel_core::menus::inventory::InventoryMenu;
use crate::events::prelude::Render;

pub fn handle_inventory_menu_spawned(
    e_inventory_menu_added: On<Add, InventoryMenu>,
    mut commands: Commands,
) {
    info!("Inventory Menu Spawned");

    commands.entity(e_inventory_menu_added.entity).observe(handle_render_inventory).trigger(Render::new);
}
