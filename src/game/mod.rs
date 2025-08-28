//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use crate::game::player::components::prelude::Player;
use crate::screens::Screen;

pub mod animation;
mod dialogue;
pub mod level;
mod lighting;
mod movement;
mod path_finding;
mod player;
mod scene;
mod trigger;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .add_plugins(FilterQueryInspectorPlugin::<With<Player>>::default())
        .add_plugins((
            level::plugin,
            trigger::plugin,
            movement::plugin,
            player::plugin,
            animation::plugin,
            dialogue::plugin,
            path_finding::plugin,
            scene::plugin,
            lighting::plugin,
        ))
        .configure_sets(
            PreUpdate,
            GameplaySystems.run_if(in_state(Screen::Gameplay)),
        )
        .configure_sets(Update, GameplaySystems.run_if(in_state(Screen::Gameplay)))
        .configure_sets(
            FixedUpdate,
            GameplaySystems.run_if(in_state(Screen::Gameplay)),
        );
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct GameplaySystems;
