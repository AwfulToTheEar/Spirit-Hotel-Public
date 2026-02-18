use bevy::prelude::*;
use spirit_hotel_level::components::prelude::ReparentToWorld;
use spirit_hotel_player::components::prelude::Player;

pub fn reparent_player_to_world(
    e_player_added: On<Add, Player>,
    mut commands: Commands,
) {
    commands.entity(e_player_added.entity).insert(ReparentToWorld);
}
