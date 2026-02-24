use crate::components::prelude::{InventoryItemId, InventoryItemRef};
use crate::resources::prelude::{InventoryManifest};
use bevy::prelude::*;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::Manifest;

pub fn handle_inventory_item_ref_add_inventory_item(
    e_inventory_item_ref_added: On<Add, InventoryItemRef>,
    mut commands: Commands,
    inventory_manifest: Res<InventoryManifest>,
    q_inventory_item_refs: Query<&InventoryItemRef>,
) {
    let inventory_item_ref = q_inventory_item_refs
        .get(e_inventory_item_ref_added.entity)
        .unwrap();

    let id = Id::from_name(&inventory_item_ref.0);
    let Some(inventory_item) = inventory_manifest.get(id) else {
        return;
    };

    commands
        .entity(e_inventory_item_ref_added.entity)
        .remove::<InventoryItemRef>()
        .insert(
            (
                InventoryItemId(id),
                inventory_item.clone()
            )
        );
}
