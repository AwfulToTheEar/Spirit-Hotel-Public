use bevy::prelude::*;

use crate::components::prelude::Player;

pub fn apply_player_translate_to_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    s_player: Single<&GlobalTransform, (With<Player>, Changed<GlobalTransform>)>,
) {
    let camera_transform = camera.as_mut();

    let player_transform = s_player.into_inner();

    camera_transform.translation = player_transform.translation();
}
