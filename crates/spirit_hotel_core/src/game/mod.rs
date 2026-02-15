//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

mod show_image;
mod teleport;

use crate::screens::Screen;
use bevy::prelude::*;
pub use show_image::*;
pub use teleport::*;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(show_image::plugin)
        .configure_sets(
            PreUpdate,
            GameplaySystems.run_if(in_state(Screen::Gameplay)),
        )
        .configure_sets(Update, GameplaySystems.run_if(in_state(Screen::Gameplay)))
        .configure_sets(
            FixedUpdate,
            GameplaySystems.run_if(in_state(Screen::Gameplay)),
        )
        .add_systems(Update, teleport.in_set(GameplaySystems));
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct GameplaySystems;
