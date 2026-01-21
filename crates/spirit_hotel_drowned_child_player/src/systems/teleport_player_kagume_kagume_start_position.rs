use bevy::prelude::*;
use spirit_hotel_core::game::Teleport;
use spirit_hotel_drowned_child::components::prelude::{KagumeKagume, KagumeKagumeStartPosition};
use spirit_hotel_player::components::prelude::Player;

pub fn teleport_player_kagume_kagume_start_position(
    _: On<Add, KagumeKagume>,
    mut commands: Commands,
    s_start_position: Single<&GlobalTransform, (With<KagumeKagumeStartPosition>, Without<Player>)>,
    mut s_player: Single<Entity, With<Player>>,
) {
    let target_transform = s_start_position.into_inner();
    let entity = s_player.into_inner();

    info!("Teleporting player to start position. ({})", target_transform.translation());

    commands.entity(entity).insert(Teleport(target_transform.translation()));
}
