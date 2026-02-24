use crate::components::prelude::RemoveItemFromRef;
use bevy::prelude::*;
use leafwing_manifest::identifier::Id;
use spirit_hotel_core::menus::inventory::InventoryMenu;
use spirit_hotel_inventory::components::prelude::{Inventory, InventoryItem};
use spirit_hotel_inventory::events::prelude::Render;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::events::prelude::TriggerActivated;

pub fn remove_item_from_ref_on_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    mut s_inventory: Single<&mut Inventory>,
    q_menu: Query<Entity, With<InventoryMenu>>,
    q_triggers: Query<&RemoveItemFromRef, With<IsTrigger>>,
) {
    let Ok(remove_item_from_ref) = q_triggers.get(e_trigger_activated.trigger) else {
        return;
    };

    let id_to_remove = Id::<InventoryItem>::from_name(&remove_item_from_ref.0);

    let mut inventory = s_inventory.into_inner();

    let Some(index) = inventory.0.iter().position(|id| *id == id_to_remove) else {
        return;
    };

    inventory.0.remove(index);

    let Ok(menu) = q_menu.single() else {
        return;
    };

    commands.trigger(Render(menu))
}
