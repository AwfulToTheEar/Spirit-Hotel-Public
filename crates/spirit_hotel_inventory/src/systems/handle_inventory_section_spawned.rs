use bevy::prelude::*;
use spirit_hotel_core::menus::inventory::InventoryMenu;
use crate::components::prelude::InventoryMenuSections;
use crate::events::prelude::{Render, SectionSpawned};

pub fn handle_inventory_section_spawned(
    e_spawned: On<SectionSpawned>,
    mut commands: Commands,
    s_menu: Single<&mut InventoryMenuSections, With<InventoryMenu>>,
) {
    info!("Section spawned: {:?}", e_spawned.entity);

    let mut sections = s_menu.into_inner();

    sections.0.insert(e_spawned.section.clone(), e_spawned.entity);

    commands.entity(e_spawned.entity).trigger(Render::new);
}