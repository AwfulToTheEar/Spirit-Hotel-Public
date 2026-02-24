use crate::components::prelude::InventoryItem;
use crate::resources::prelude::InventoryManifest;
use bevy::prelude::*;

pub fn handle_inventory_item_node_add_visuals(
    e_inventory_item_added: On<Add, InventoryItem>,
    mut commands: Commands,
    inventory_manifest: Res<InventoryManifest>,
    q_inventory_items: Query<&InventoryItem, With<Node>>,
) {
    let Ok(inventory_item) = q_inventory_items.get(e_inventory_item_added.entity) else {
        return;
    };

    commands
        .entity(e_inventory_item_added.entity)
        .insert(ImageNode::from_atlas_image(
            inventory_manifest.texture.clone(),
            TextureAtlas::from(inventory_manifest.layout.clone()).with_index(inventory_item.index),
        ));
}
