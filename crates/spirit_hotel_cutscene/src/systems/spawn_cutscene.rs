use crate::resources::prelude::FadeToCutscene;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy_av1::{PlaybackMode, VideoPlayer, VideoSink};
use spirit_hotel_core::fades::Fade;
use spirit_hotel_core::theme::widget;

pub fn spawn_cutscene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fade_to_cutscene: Res<FadeToCutscene>,
) {
    commands
        .spawn((
            widget::ui_root("Cutscene"),
            RenderLayers::layer(1),
            VideoPlayer::new(
                asset_server.load(fade_to_cutscene.0.clone()),
                PlaybackMode::Despawn,
            ),
        ))
        .observe(
            |add: On<Add, VideoSink>, sinks: Query<&VideoSink>, mut commands: Commands| {
                let entity = add.entity;
                if let Ok(sink) = sinks.get(entity) {
                    commands
                        .entity(entity)
                        .insert(ImageNode::new(sink.image().clone()));
                }
            },
        );
}
