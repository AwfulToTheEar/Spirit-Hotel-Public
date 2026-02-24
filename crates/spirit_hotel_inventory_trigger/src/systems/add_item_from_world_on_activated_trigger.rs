use bevy::prelude::*;
use spirit_hotel_core::menus::inventory::InventoryMenu;
use spirit_hotel_inventory::components::prelude::{Inventory, InventoryItemId};
use spirit_hotel_inventory::events::prelude::Render;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::events::prelude::TriggerActivated;
use crate::components::prelude::{AddItemFromWorld};

pub fn add_item_from_world_on_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    mut s_inventory: Single<&mut Inventory>,
    q_menu: Query<Entity, With<InventoryMenu>>,
    q_triggers: Query<&AddItemFromWorld, With<IsTrigger>>,
    q_inventory_items: Query<(Entity, &InventoryItemId), With<Transform>>,
) {
    let Ok(add_item_from_world) = q_triggers.get(e_trigger_activated.trigger) else {
        return;
    };

    let Ok((item, inventory_item_id)) = q_inventory_items.get(add_item_from_world.0) else { return; };

    commands.entity(item).despawn();

    let mut inventory = s_inventory.into_inner();

    inventory.0.push(inventory_item_id.0);

    let Ok(menu) = q_menu.single() else {
        return;
    };

    commands.trigger(Render(menu))
}