//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_northstar::{debug::NorthstarDebugPlugin, prelude::CardinalNeighborhood};
use bevy_mod_debugdump::CommandLineArgs;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        EguiPlugin { enable_multipass_for_primary_context: true },
        WorldInspectorPlugin::new(),
        //TiledDebugPluginGroup,
        NorthstarDebugPlugin::<CardinalNeighborhood>::default(),
        //PhysicsDebugPlugin::default(),
        CommandLineArgs
    ));
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
