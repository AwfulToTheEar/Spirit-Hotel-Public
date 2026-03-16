use crate::components::prelude::{InventoryItem, SelectedItemId};
use crate::events::prelude::Render;
use crate::resources::prelude::InventoryManifest;
use bevy::prelude::*;
use leafwing_manifest::manifest::Manifest;
use spirit_hotel_core::menus::inventory::InventoryMenu;
use spirit_hotel_core::theme::widget;

pub fn handle_render_selected_inventory_item(
    e_render: On<Render>,
    mut commands: Commands,
    inventory_manifest: Res<InventoryManifest>,
    s_menu: Single<Option<&SelectedItemId>, With<InventoryMenu>>,
) {
    info!("Render Selected Inventory Item");

    let option_selected_item_id = s_menu.into_inner();

    let mut option_inventory_item: Option<InventoryItem> = None;

    if let Some(selected_item_id) = option_selected_item_id
        && let Some(inventory_item) = inventory_manifest.get(selected_item_id.0)
    {
        option_inventory_item = Some(inventory_item.clone());
    };

    commands.entity(e_render.0).despawn_children().insert((
        Name::from("Selected Inventory Item"),
        Node {
            display: Display::Block,
            ..default()
        },
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            let Some(inventory_item) = option_inventory_item else {
                return;
            };

            parent.spawn(widget::label(inventory_item.name));
            parent.spawn(widget::label("Description"));
            parent.spawn(Text::from(inventory_item.description));
        })),
    ));
}
