// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

#[cfg(feature = "dev")]
mod dev_tools;

use spirit_hotel_core::bevy::{asset::AssetMetaCheck, prelude::*};
use spirit_hotel_lighting::light_2d::light::{AmbientLight2d, Light2d};

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Spirit Hotel".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            spirit_hotel_core::plugin,
            spirit_hotel_level::plugin,
            spirit_hotel_dialogue::plugin,
            spirit_hotel_lighting::plugin,
            spirit_hotel_pathfinding::plugin,
            spirit_hotel_movement::plugin,
            spirit_hotel_animation::plugin,
            spirit_hotel_player::plugin,
            spirit_hotel_scene::plugin,
            spirit_hotel_trigger::plugin,

            spirit_hotel_level_animation::plugin,
            spirit_hotel_level_movement::plugin,
            spirit_hotel_level_player::plugin,
            spirit_hotel_level_pathfinding::plugin,
            spirit_hotel_level_trigger::plugin,

            spirit_hotel_dialogue_trigger::plugin,

            spirit_hotel_pathfinding_movement::plugin,
            spirit_hotel_pathfinding_movement_trigger::plugin,
            spirit_hotel_pathfinding_trigger::plugin,

            spirit_hotel_movement_trigger::plugin,

            spirit_hotel_animation_movement::plugin,

            spirit_hotel_player_movement::plugin,

            spirit_hotel_scene_dialogue::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
        ));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Light2d {
            ambient_light: AmbientLight2d {
                brightness: 0.1,
                ..default()
            },
        },
    ));
}
