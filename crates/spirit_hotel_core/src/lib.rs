pub mod asset_tracking;
pub mod audio;
pub mod dismissibles;
pub mod fades;
pub mod game;
pub mod general_controls;
pub mod menus;
pub mod screens;
pub mod theme;

use crate::dismissibles::DismissiblesOpen;
use crate::fades::Fade;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_pretty_text::PrettyTextPlugin;
use leafwing_manifest::asset_state::SimpleAssetState;
use leafwing_manifest::plugin::ManifestPlugin;

pub struct SpiritHotelCorePlugin;

impl Plugin for SpiritHotelCorePlugin {
    fn build(&self, app: &mut App) {
        // Add other plugins.
        app.init_state::<SimpleAssetState>();
        app.add_plugins((
            EnhancedInputPlugin,
            ManifestPlugin::<SimpleAssetState>::default(),
            PrettyTextPlugin,
        ));
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            game::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
            fades::plugin,
            dismissibles::plugin,
            general_controls::plugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        app.configure_sets(
            FixedUpdate,
            (AppSystems::TickTimers, AppSystems::Update).chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(
            PreUpdate,
            PausableSystems.run_if(
                in_state(Pause(false))
                    .and(in_state(Fade::None))
                    .and(in_state(DismissiblesOpen(false))),
            ),
        );
        app.configure_sets(
            Update,
            PausableSystems.run_if(
                in_state(Pause(false))
                    .and(in_state(Fade::None))
                    .and(in_state(DismissiblesOpen(false))),
            ),
        );
        app.configure_sets(
            FixedUpdate,
            PausableSystems.run_if(
                in_state(Pause(false))
                    .and(in_state(Fade::None))
                    .and(in_state(DismissiblesOpen(false))),
            ),
        );

        app.add_systems(Startup, spawn_cameras);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct OverlayRoot;

fn spawn_cameras(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera {
            order: 0,
            ..default()
        },
        Camera2d,
        MainCamera,
        RenderLayers::layer(0),
    ));

    commands.spawn((
        Name::new("Overlay Camera"),
        Camera {
            order: 1,
            ..default()
        },
        Camera2d,
        RenderLayers::layer(1),
    ));
}
