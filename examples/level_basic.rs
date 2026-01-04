use bevy::prelude::*;
use spirit_hotel_core::SpiritHotelCorePlugin;
use spirit_hotel_level::SpiritHotelLevelPlugin;

fn main() {
    let mut app = App::new();
    app
        // Bevy default plugins: prevent blur effect by changing default sampling.
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        // Add bevy_ecs_tiled plugin: bevy_ecs_tilemap::TilemapPlugin will
        // be automatically added as well if it's not already done.
        .add_plugins((
            SpiritHotelCorePlugin,
            SpiritHotelLevelPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
    ));
}