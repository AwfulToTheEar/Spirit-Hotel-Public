use bevy::prelude::*;
use spirit_hotel_inventory::components::prelude::Inventory;
use spirit_hotel_player::components::prelude::Player;

pub fn add_player_inventory(e_player_added: On<Add, Player>, mut commands: Commands) {
    commands
        .entity(e_player_added.entity)
        .insert(Inventory::default());
}
