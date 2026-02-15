use bevy::prelude::*;
use bevy_light_2d::light::{AmbientLight2d, Light2d};
use spirit_hotel_core::MainCamera;

pub fn add_ambient_light_to_main_camera(
    e_add_main_camera: On<Add, MainCamera>,
    mut commands: Commands,
) {
    commands.entity(e_add_main_camera.entity).insert(Light2d {
        ambient_light: AmbientLight2d {
            brightness: 0.1,
            ..default()
        },
    });
}
