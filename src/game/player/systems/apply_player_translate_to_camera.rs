use bevy::prelude::*;

use crate::game::player::components::prelude::Player;

pub fn apply_player_translate_to_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    player_option: Option<Single<&GlobalTransform, (With<Player>, Changed<GlobalTransform>)>>,
) {
    let Some(player) = player_option else {
        return;
    };

    let camera_transform = camera.as_mut();

    let player_transform = player.into_inner();

    camera_transform.translation = player_transform.translation();
}
