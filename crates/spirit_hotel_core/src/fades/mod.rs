mod fade_in;
mod fade_out;

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use crate::{screens::Screen, theme::palette::FADE_BACKGROUND_COLOR};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Fade>();
    app.init_resource::<FadeData>();

    app.add_plugins((fade_in::plugin, fade_out::plugin))
        .add_systems(Startup, spawn_fade_camera);
}

#[derive(Resource)]
pub struct FadeData {
    pub color: Color,
    pub duration: f32
}

#[derive(Resource)]
pub struct FadeToScreen(pub Screen);

#[derive(Resource)]
pub struct FadeOutInstantly;

impl FromWorld for FadeData {
    fn from_world(_world: &mut World) -> Self {
        Self {
            color: FADE_BACKGROUND_COLOR,
            duration: 0.3
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

fn spawn_fade_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Fade Camera"),
        Camera {
            order: 1,
            ..default()
        },
        Camera2d,
        RenderLayers::layer(1)
    ));
}