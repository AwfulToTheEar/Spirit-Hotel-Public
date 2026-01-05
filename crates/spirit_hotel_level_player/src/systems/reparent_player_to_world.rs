use bevy::prelude::*;
use spirit_hotel_level::tiled::prelude::{TiledWorld};
use spirit_hotel_player::components::prelude::Player;

pub fn reparent_player_to_world(
    mut commands: Commands,
    s_world: Single<Entity, With<TiledWorld>>,
    s_player: Single<Entity, Added<Player>>,
) {
    let world = s_world.into_inner();

    let player = s_player.into_inner();

    commands.entity(player).set_parent_in_place(world);
}
