use crate::components::prelude::{InventoryMenuSection, InventoryMenuSections, ItemToCombine};
use crate::events::prelude::Render;
use bevy::prelude::*;
use spirit_hotel_core::menus::inventory::InventoryMenu;

pub fn handle_selected_item_combine_with_cancel(
    _: On<Pointer<Click>>,
    mut commands: Commands,
    s_menu: Single<(Entity, &InventoryMenuSections), (With<InventoryMenu>, With<ItemToCombine>)>,
) {
    let (menu, sections) = s_menu.into_inner();

    commands.entity(menu).remove::<ItemToCombine>();
    commands
        .entity(*sections.get(InventoryMenuSection::Actions).unwrap())
        .trigger(Render::new);
}
