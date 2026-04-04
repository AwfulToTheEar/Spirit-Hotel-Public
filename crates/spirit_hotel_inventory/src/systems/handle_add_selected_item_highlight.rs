use crate::components::prelude::{InventoryItemId, SelectedItemId};
use bevy::prelude::*;
use spirit_hotel_core::menus::inventory::InventoryMenu;

pub fn handle_add_selected_item_highlight(
    e_add_inventory_item_id: On<Add, InventoryItemId>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    s_selected_item_id: Single<&SelectedItemId, With<InventoryMenu>>,
    q_inventory_items: Query<&InventoryItemId, With<Node>>,
) {
    info!("Add Selected Item Highlight");

    let selected_item_id = s_selected_item_id.into_inner();

    let Ok(inventory_item_id) = q_inventory_items.get(e_add_inventory_item_id.entity) else {
        return;
    };

    if inventory_item_id.0 != selected_item_id.0 {
        return;
    }

    let ui_image = asset_server.load("images/FreeHorrorUi.png");

    commands
        .entity(e_add_inventory_item_id.entity)
        .insert(children![(
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ImageNode {
                image: ui_image.clone(),
                rect: Some(Rect {
                    min: Vec2::new(198., 132.),
                    max: Vec2::new(222., 157.)
                }),
                image_mode: NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(9.),
                    max_corner_scale: 6.,
                    center_scale_mode: SliceScaleMode::Stretch,
                    sides_scale_mode: SliceScaleMode::Stretch,
                    ..default()
                }),
                ..default()
            },
            ZIndex(2)
        )]);
}
