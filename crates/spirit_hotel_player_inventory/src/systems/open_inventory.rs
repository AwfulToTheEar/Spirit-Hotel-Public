use crate::inputs::prelude::OpenInventory;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Start};
use spirit_hotel_core::menus::Menu;

pub fn open_inventory(_: On<Start<OpenInventory>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Inventory);
}
