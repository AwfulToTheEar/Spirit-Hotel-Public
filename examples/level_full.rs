use bevy::prelude::*;
use spirit_hotel_core::SpiritHotelCorePlugin;
use spirit_hotel_level::SpiritHotelLevelPlugin;
use spirit_hotel_animation::SpiritHotelAnimationPlugin;
use spirit_hotel_level_animation::SpiritHotelLevelAnimationPlugin;
use spirit_hotel_level_movement::SpiritHotelLevelMovementPlugin;
use spirit_hotel_level_pathfinding::SpiritHotelLevelPathfindingPlugin;
use spirit_hotel_level_player::SpiritHotelLevelPlayerPlugin;
use spirit_hotel_level_trigger::SpiritHotelLevelTriggerPlugin;
use spirit_hotel_movement::SpiritHotelMovementPlugin;
use spirit_hotel_pathfinding::SpiritHotelPathfindingPlugin;
use spirit_hotel_player::SpiritHotelPlayerPlugin;
use spirit_hotel_trigger::SpiritHotelTriggerPlugin;

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
            SpiritHotelAnimationPlugin,
            SpiritHotelMovementPlugin,
            SpiritHotelPathfindingPlugin,
            SpiritHotelPlayerPlugin,
            SpiritHotelTriggerPlugin,

            SpiritHotelLevelTriggerPlugin,
            SpiritHotelLevelPathfindingPlugin,
            SpiritHotelLevelPlayerPlugin,
            //SpiritHotelLevelMovementPlugin,
            SpiritHotelLevelAnimationPlugin
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