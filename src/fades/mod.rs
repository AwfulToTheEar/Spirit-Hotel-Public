mod fade_in;
mod fade_out;

use bevy::{prelude::*};

use crate::{screens::Screen, theme::palette::FADE_BACKGROUND_COLOR};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Fade>();
    app.init_resource::<FadeData>();

    app.add_plugins((
        fade_in::plugin,
        fade_out::plugin,
    ));
}

#[derive(Resource)]
pub struct FadeData {
    pub color: Color,
    pub duration: f32,
    pub next_state: Option<Screen>
}

impl FromWorld for FadeData {
    fn from_world(_world: &mut World) -> Self {
        return Self {
            color: FADE_BACKGROUND_COLOR,
            duration: 0.3,
            next_state: None
        }
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Fade {
    #[default]
    None,
    In,
    Out,
}
