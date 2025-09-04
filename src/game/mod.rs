//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use crate::screens::Screen;
use bevy::prelude::*;

pub mod animation;
mod dialogue;
mod drowned_child;
mod general;
pub mod level;
mod lighting;
mod movement;
mod path_finding;
mod player;
mod scene;
mod trigger;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        general::plugin,
        level::plugin,
        trigger::plugin,
        movement::plugin,
        player::plugin,
        animation::plugin,
        dialogue::plugin,
        path_finding::plugin,
        scene::plugin,
        lighting::plugin,
        drowned_child::plugin,
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
