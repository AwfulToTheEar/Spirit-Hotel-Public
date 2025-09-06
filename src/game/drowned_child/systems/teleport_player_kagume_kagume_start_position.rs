use crate::game::drowned_child::components::prelude::KagumeKagumeStartPosition;
use crate::game::player::components::prelude::Player;
use bevy::prelude::*;

pub fn teleport_player_kagume_kagume_start_position(
    s_start_position: Single<&Transform, (With<KagumeKagumeStartPosition>, Without<Player>)>,
    mut s_player: Single<&mut Transform, With<Player>>,
) {
    let target_transform = s_start_position.into_inner();
    let player_transform = s_player.as_mut();

    player_transform.translation = target_transform.translation
}
