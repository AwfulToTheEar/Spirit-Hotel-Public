use crate::resources::prelude::SpawnCutscene;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy_av1::{PlaybackMode, VideoPlayer, VideoSink};
use spirit_hotel_core::fades::FadeTasks;
use spirit_hotel_core::theme::widget;

pub fn spawn_cutscene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_cutscene: Res<SpawnCutscene>,
) {
    commands
        .spawn((
            widget::ui_root("Cutscene"),
            RenderLayers::layer(2),
            VideoPlayer::new(
                asset_server.load(spawn_cutscene.0.clone()),
                PlaybackMode::Despawn,
            ),
        ))
        .observe(
            |add: On<Add, VideoSink>, sinks: Query<&VideoSink>, mut commands: Commands| {
                info!("Added video sink: {:?}", add.entity);
                let entity = add.entity;
                if let Ok(sink) = sinks.get(entity) {
                    commands
                        .entity(entity)
                        .insert(ImageNode::new(sink.image().clone()));
                }
            },
        );
}
