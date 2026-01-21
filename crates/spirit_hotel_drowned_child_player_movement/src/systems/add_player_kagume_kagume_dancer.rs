use bevy::prelude::*;
use spirit_hotel_drowned_child::components::prelude::KagumeKagume;
use spirit_hotel_drowned_child_movement::components::prelude::KagumeKagumeDancer;
use spirit_hotel_player::components::prelude::Player;

pub fn add_player_kagume_kagume_dancer(
    _: On<Add, KagumeKagume>,
    mut commands: Commands,
    s_player: Single<Entity, With<Player>>,
) {
    let entity = s_player.into_inner();

    commands.entity(entity).insert(KagumeKagumeDancer);
}
