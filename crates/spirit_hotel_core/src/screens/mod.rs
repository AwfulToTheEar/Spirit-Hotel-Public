//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;
use crate::fades::{ChangeScreen, FadeTaskFinished};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[reflect(Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}

fn complete_fade_to_screen_task(mut commands: Commands) {
    commands.trigger(FadeTaskFinished::<ChangeScreen>::new());
}
