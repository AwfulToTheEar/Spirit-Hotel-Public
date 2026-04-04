use crate::components::prelude::{AddItemFromRef};
use bevy::prelude::*;
use leafwing_manifest::identifier::Id;
use spirit_hotel_core::menus::inventory::InventoryMenu;
use spirit_hotel_inventory::components::prelude::{Inventory, InventoryItem};
use spirit_hotel_inventory::events::prelude::Render;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::events::prelude::TriggerActivated;

pub fn add_item_from_ref_on_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    s_inventory: Single<&mut Inventory>,
    q_menu: Query<Entity, With<InventoryMenu>>,
    q_triggers: Query<&AddItemFromRef, With<IsTrigger>>,
) {
    let Ok(add_item_from_ref) = q_triggers.get(e_trigger_activated.trigger) else {
        return;
    };

    let id_to_add = Id::<InventoryItem>::from_name(&add_item_from_ref.0);

    let mut inventory = s_inventory.into_inner();

    inventory.0.push(id_to_add);

    let Ok(menu) = q_menu.single() else {
        return;
    };

    commands.trigger(Render(menu))
}
