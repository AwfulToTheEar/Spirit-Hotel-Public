use crate::components::prelude::{InventoryMenuSection, InventoryMenuSections, ItemToCombine, SelectedItemId};
use crate::events::prelude::Render;
use bevy::prelude::*;
use spirit_hotel_core::menus::inventory::InventoryMenu;

pub fn handle_selected_item_combine_with(
    _: On<Pointer<Click>>,
    mut commands: Commands,
    s_menu: Single<(Entity, &InventoryMenuSections, &SelectedItemId), With<InventoryMenu>>,
) {
    let (menu, sections, selected_item) = s_menu.into_inner();

    commands.entity(menu).insert(ItemToCombine(selected_item.0));
    commands.entity(*sections.get(InventoryMenuSection::Actions).unwrap()).trigger(Render::new);
}
