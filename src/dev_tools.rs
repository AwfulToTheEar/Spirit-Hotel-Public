//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_mod_debugdump::CommandLineArgs;
use spirit_hotel_core::Pause;
use spirit_hotel_core::fades::Fade;
use spirit_hotel_core::screens::Screen;
use spirit_hotel_inventory::components::prelude::InventoryItem;

pub(super) struct SpiritHotelDevToolsPlugin;

impl Plugin for SpiritHotelDevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            FilterQueryInspectorPlugin::<(With<InventoryItem>,)>::new(),
            WorldInspectorPlugin::new(),
            //TiledDebugPluginGroup,
            //NorthstarDebugPlugin::<CardinalNeighborhood>::default(),
            //PhysicsDebugPlugin::default(),
            CommandLineArgs,
        ));
        // Log `Screen` state transitions.
        app.add_systems(Update, log_transitions::<Screen>);
        app.add_systems(Update, log_transitions::<Fade>);
        app.add_systems(Update, log_transitions::<Pause>);

        // Toggle the debug overlay for UI.
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        );
    }
}

const TOGGLE_KEY: KeyCode = KeyCode::Backspace;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
