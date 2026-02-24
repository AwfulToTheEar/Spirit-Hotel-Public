use crate::components::prelude::{InventoryMenuSection, InventoryMenuSections};
use crate::events::prelude::{Render, SectionSpawned};
use crate::systems::render::*;
use bevy::prelude::*;

pub fn handle_render_inventory(e_render: On<Render>, mut commands: Commands) {
    info!("Render Inventory");

    commands
        .entity(e_render.0)
        .despawn_children()
        .insert(InventoryMenuSections::default())
        .insert(children![(
            Node {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.), GridTrack::fr(1.)],
                column_gap: Val::Px(10.),
                ..default()
            },
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                parent
                    .spawn_empty()
                    .observe(handle_render_selected_inventory_item)
                    .trigger(SectionSpawned::with_section(InventoryMenuSection::SelectedInventoryItem));

                parent.spawn((
                    Node {
                        display: Display::Block,
                        ..default()
                    },
                    Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                        parent
                            .spawn_empty()
                            .observe(handle_render_inventory_items)
                            .trigger(SectionSpawned::with_section(InventoryMenuSection::InventoryItems));

                        parent
                            .spawn_empty()
                            .observe(handle_render_inventory_actions)
                            .trigger(SectionSpawned::with_section(InventoryMenuSection::Actions));
                    })),
                ));
            }))
        )]);
}
