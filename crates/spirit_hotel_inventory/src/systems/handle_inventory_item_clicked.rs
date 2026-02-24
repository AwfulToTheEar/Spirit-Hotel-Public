use crate::components::prelude::{InventoryItem, InventoryItemId, SelectedItemId};
use bevy::prelude::*;
use spirit_hotel_core::menus::inventory::InventoryMenu;

pub fn handle_inventory_item_clicked(
    e_clicked_item: On<Pointer<Click>>,
    mut commands: Commands,
    q_inventory_items: Query<&InventoryItemId>,
    s_inventory_menu: Single<Entity, With<InventoryMenu>>,
) {
    let Ok(inventory_item_id) = q_inventory_items.get(e_clicked_item.entity) else {
        return;
    };

    let inventory_menu = s_inventory_menu.into_inner();

    commands
        .entity(inventory_menu)
        .insert(SelectedItemId(inventory_item_id.0));
}
