use crate::components::prelude::{Inventory, ItemToCombine, SelectedItemId};
use crate::resources::prelude::InventoryManifest;
use bevy::prelude::*;
use spirit_hotel_core::menus::inventory::InventoryMenu;

pub fn handle_selected_item_combine(
    _: On<Pointer<Click>>,
    mut commands: Commands,
    inventory_manifest: Res<InventoryManifest>,
    s_menu: Single<(Entity, &SelectedItemId, &ItemToCombine), With<InventoryMenu>>,
    s_inventory: Single<&mut Inventory>,
) {
    let (menu, selected_item_id, item_to_combine) = s_menu.into_inner();

    if selected_item_id.0 == item_to_combine.0 {
        return;
    }

    let Some((_, item_id)) =
        inventory_manifest
            .combinations
            .iter()
            .find(|((input_a, input_b), _)| {
                if (input_a == &selected_item_id.0 || input_a == &item_to_combine.0)
                    && (input_b == &selected_item_id.0 || input_b == &item_to_combine.0)
                {
                    return true;
                }

                return false;
            })
    else {
        return;
    };

    let mut inventory = s_inventory.into_inner();

    inventory.0 = inventory
        .0
        .clone()
        .into_iter()
        .filter(|id| id != &selected_item_id.0 && id != &item_to_combine.0)
        .collect::<Vec<_>>();

    inventory.0.push(item_id.clone());

    commands
        .entity(menu)
        .remove::<ItemToCombine>()
        .insert(SelectedItemId(item_id.clone()));
}
