use bevy::prelude::*;
use leafwing_manifest::manifest::Manifest;
use crate::components::prelude::{Inventory, InventoryItemId};
use crate::events::prelude::Render;
use crate::resources::prelude::InventoryManifest;
use crate::systems::prelude::handle_inventory_item_clicked;

pub fn handle_render_inventory_items(
    e_render: On<Render>,
    mut commands: Commands,
    inventory_manifest: Res<InventoryManifest>,
    s_inventory: Single<&Inventory>,
) {
    let inventory = s_inventory.into_inner();

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
                Node::default(),
                Button,
            ))
        })
        .collect::<Vec<_>>();
    
    commands.entity(e_render.0).despawn_children().insert((
        Name::from("Inventory Items"),
        Node {
            display: Display::Grid,
            grid_template_columns: vec![
                GridTrack::fr(1.),
                GridTrack::fr(1.),
                GridTrack::fr(1.),
            ],
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
        }))
    ));
}