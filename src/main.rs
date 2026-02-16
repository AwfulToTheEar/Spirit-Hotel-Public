// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

#[cfg(feature = "dev")]
mod dev_tools;

#[cfg(feature = "dev")]
use crate::dev_tools::SpiritHotelDevToolsPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy::log::LogPlugin;
use spirit_hotel_ai::SpiritHotelAiPlugin;
use spirit_hotel_ai_all::SpiritHotelAiAllPlugin;
use spirit_hotel_ai_pathfinding::SpiritHotelAiPathfindingPlugin;
use spirit_hotel_ai_pathfinding_movement::SpiritHotelAiPathfindingMovementPlugin;
use spirit_hotel_ai_player::SpiritHotelAiPlayerPlugin;
use spirit_hotel_animation::SpiritHotelAnimationPlugin;
use spirit_hotel_animation_movement::SpiritHotelAnimationMovementPlugin;
use spirit_hotel_core::SpiritHotelCorePlugin;
use spirit_hotel_cutscene::SpiritHotelCutscenePlugin;
use spirit_hotel_cutscene_trigger::SpiritHotelCutsceneTriggerPlugin;
use spirit_hotel_dialogue::SpiritHotelDialoguePlugin;
use spirit_hotel_dialogue_trigger::SpiritHotelDialogueTriggerPlugin;
use spirit_hotel_drowned_child::SpiritHotelDrownedChildPlugin;
use spirit_hotel_drowned_child_movement::SpiritHotelDrownedChildMovementPlugin;
use spirit_hotel_drowned_child_player::SpiritHotelDrownedChildPlayerPlugin;
use spirit_hotel_drowned_child_player_movement::SpiritHotelDrownedChildPlayerMovementPlugin;
use spirit_hotel_drowned_child_player_pathfinding::SpiritHotelDrownedChildPlayerPathfindingPlugin;
use spirit_hotel_drowned_child_player_pathfinding_movement::SpiritHotelDrownedChildPlayerPathfindingMovementPlugin;
use spirit_hotel_level::SpiritHotelLevelPlugin;
use spirit_hotel_level_animation::SpiritHotelLevelAnimationPlugin;
use spirit_hotel_level_drowned_child::SpiritHotelLevelDrownedChildPlugin;
use spirit_hotel_level_movement::SpiritHotelLevelMovementPlugin;
use spirit_hotel_level_pathfinding::SpiritHotelLevelPathfindingPlugin;
use spirit_hotel_level_player::SpiritHotelLevelPlayerPlugin;
use spirit_hotel_level_scene::SpiritHotelLevelScenePlugin;
use spirit_hotel_level_trigger::SpiritHotelLevelTriggerPlugin;
use spirit_hotel_lighting::SpiritHotelLightingPlugin;
use spirit_hotel_movement::SpiritHotelMovementPlugin;
use spirit_hotel_movement_trigger::SpiritHotelMovementTriggerPlugin;
use spirit_hotel_pathfinding::SpiritHotelPathfindingPlugin;
use spirit_hotel_pathfinding_movement::SpiritHotelPathfindingMovementPlugin;
use spirit_hotel_pathfinding_movement_trigger::SpiritHotelPathfindingMovementTriggerPlugin;
use spirit_hotel_pathfinding_trigger::SpiritHotelPathfindingTriggerPlugin;
use spirit_hotel_player::SpiritHotelPlayerPlugin;
use spirit_hotel_player_movement::SpiritHotelPlayerMovementPlugin;
use spirit_hotel_scene::SpiritHotelScenePlugin;
use spirit_hotel_scene_dialogue::SpiritHotelSceneDialoguePlugin;
use spirit_hotel_scene_trigger::SpiritHotelSceneTriggerPlugin;
use spirit_hotel_trigger::SpiritHotelTriggerPlugin;

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
                })
                .set(LogPlugin {
                    //filter: "dogoap=debug,bevy_dogoap=debug".to_string(),
                    ..default()
                })
        );

        app.add_plugins((
            SpiritHotelCorePlugin,
            SpiritHotelLevelPlugin,
            SpiritHotelDialoguePlugin,
            SpiritHotelLightingPlugin,
            SpiritHotelPathfindingPlugin,
            SpiritHotelMovementPlugin,
            SpiritHotelAnimationPlugin,
            SpiritHotelPlayerPlugin,
            SpiritHotelScenePlugin,
            SpiritHotelTriggerPlugin,
            SpiritHotelDrownedChildPlugin,
            SpiritHotelCutscenePlugin,
            SpiritHotelAiPlugin,
            #[cfg(feature = "dev")]
            SpiritHotelDevToolsPlugin,
        ));

        // Add other plugins.
        app.add_plugins((
            SpiritHotelLevelAnimationPlugin,
            SpiritHotelLevelMovementPlugin,
            SpiritHotelLevelPlayerPlugin,
            SpiritHotelLevelPathfindingPlugin,
            SpiritHotelLevelTriggerPlugin,
            SpiritHotelLevelScenePlugin,
            SpiritHotelLevelDrownedChildPlugin,
        ))
        .add_plugins(SpiritHotelDialogueTriggerPlugin)
        .add_plugins((
            SpiritHotelPathfindingMovementPlugin,
            SpiritHotelPathfindingMovementTriggerPlugin,
            SpiritHotelPathfindingTriggerPlugin,
        ))
        .add_plugins(SpiritHotelMovementTriggerPlugin)
        .add_plugins(SpiritHotelAnimationMovementPlugin)
        .add_plugins(SpiritHotelPlayerMovementPlugin)
        .add_plugins((
            SpiritHotelSceneDialoguePlugin,
            SpiritHotelSceneTriggerPlugin,
        ))
        .add_plugins((
            SpiritHotelDrownedChildMovementPlugin,
            SpiritHotelDrownedChildPlayerPlugin,
            SpiritHotelDrownedChildPlayerMovementPlugin,
            SpiritHotelDrownedChildPlayerPathfindingPlugin,
            SpiritHotelDrownedChildPlayerPathfindingMovementPlugin,
        ))
        .add_plugins(SpiritHotelCutsceneTriggerPlugin)
        .add_plugins((
            SpiritHotelAiPathfindingPlugin,
            SpiritHotelAiPathfindingMovementPlugin,
            SpiritHotelAiPlayerPlugin,
            SpiritHotelAiAllPlugin
        ));
    }
}
