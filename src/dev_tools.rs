//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::{FilterQueryInspectorPlugin, WorldInspectorPlugin}};
use bevy_mod_debugdump::CommandLineArgs;
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};
use spirit_hotel_core::screens::Screen;
use bevy_northstar::{
    debug::NorthstarDebugPlugin, prelude::CardinalNeighborhood,
};
use avian2d::prelude::PhysicsDebugPlugin;
use spirit_hotel_player::components::prelude::Player;

pub(super) struct SpiritHotelDevToolsPlugin;

impl Plugin for SpiritHotelDevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            FilterQueryInspectorPlugin::<(
                With<Player>,
            )>::new(),
            WorldInspectorPlugin::new(),
            //TiledDebugPluginGroup,
            NorthstarDebugPlugin::<CardinalNeighborhood>::default(),
            PhysicsDebugPlugin::default(),
            CommandLineArgs,
        ));
        // Log `Screen` state transitions.
        app.add_systems(Update, log_transitions::<Screen>);

        // Toggle the debug overlay for UI.
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        );
    }
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
