use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_tiled::prelude::{TiledObject, TiledObjectVisualOf, TiledObjectVisuals};
use spirit_hotel_inventory::components::prelude::InventoryItem;
use spirit_hotel_inventory::resources::prelude::InventoryManifest;

pub fn handle_inventory_item_tiled_object_add_visuals(
    e_inventory_item_added: On<Add, InventoryItem>,
    mut commands: Commands,
    inventory_manifest: Res<InventoryManifest>,
    q_inventory_items: Query<(&InventoryItem, &TiledObject), (With<Transform>, Without<TiledObjectVisuals>)>,
) {
    let Ok((inventory_item, tiled_object)) =
        q_inventory_items.get(e_inventory_item_added.entity)
    else {
        return;
    };

    let offset_height = match tiled_object {
        TiledObject::Rectangle { height, .. } => *height,
        _ => 0.,
    };

    commands.spawn((
        ChildOf(e_inventory_item_added.entity),
        TiledObjectVisualOf(e_inventory_item_added.entity),
        Sprite::from_atlas_image(
            inventory_manifest.texture.clone(),
            TextureAtlas::from(inventory_manifest.layout.clone()).with_index(inventory_item.index),
        ),
        Anchor::BOTTOM_LEFT,
        Transform::from_translation(Vec3::new(0., -offset_height, 0.0)),
    ));
}
