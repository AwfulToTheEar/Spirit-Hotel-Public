use crate::components::prelude::{Inventory, InventoryItem, InventoryItemId, SelectedItemId};
use crate::events::prelude::Render;
use crate::resources::prelude::InventoryManifest;
use crate::systems::handle_add_selected_item_highlight::handle_add_selected_item_highlight;
use crate::systems::prelude::handle_inventory_item_clicked;
use bevy::prelude::*;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::Manifest;
use spirit_hotel_core::menus::inventory::InventoryMenu;

pub fn handle_render_inventory_items(
    e_render: On<Render>,
    mut commands: Commands,
    inventory_manifest: Res<InventoryManifest>,
    s_inventory: Single<&Inventory>,
    s_menu: Single<Option<&SelectedItemId>, With<InventoryMenu>>,
) {
    let inventory = s_inventory.into_inner();

    let mut option_selected_item_id: Option<Id<InventoryItem>> = None;

    if let Some(selected_item_id_wrapper) = s_menu.into_inner() {
        option_selected_item_id = Some(selected_item_id_wrapper.0);
    }

    let inventory_item_buttons = inventory
        .0
        .iter()
        .filter_map(|id| {
            let Some(inventory_item) = inventory_manifest.get(*id) else {
                return None;
            };

            Some((
                Name::from(format!("Inventory Item: {}", id.raw())),
                InventoryItemId(*id),
                inventory_item.clone(),
                Button,
            ))
        })
        .collect::<Vec<_>>();

    commands.entity(e_render.0).despawn_children().insert((
        Name::from("Inventory Items"),
        Node {
            display: Display::Grid,
            grid_template_columns: vec![RepeatedGridTrack::fr(3, 1.)],
            column_gap: Val::Px(10.),
            row_gap: Val::Px(10.),
            ..default()
        },
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            for bundle in inventory_item_buttons.iter() {
                parent
                    .spawn(bundle.clone())
                    .observe(handle_inventory_item_clicked);
            }
        })),
    ));
}
