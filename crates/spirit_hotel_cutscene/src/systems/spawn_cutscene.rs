use crate::components::prelude::Cutscene;
use crate::resources::prelude::SpawnCutscene;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy_av1::{PlaybackMode, VideoPlayer, VideoSink};
use spirit_hotel_core::{OverlayCamera, OverlayRoot};

pub fn spawn_cutscene(
    e_overlay_root_added: On<Add, OverlayRoot>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    option_spawn_cutscene: Option<Res<SpawnCutscene>>,
    s_overlay_camera: Single<Entity, With<OverlayCamera>>
) {
    let Some(spawn_cutscene) = option_spawn_cutscene else {
        return;
    };

    commands
        .spawn((
            Name::new("Cutscene"),
            Node {
                width: percent(100),
                height: percent(100),
                ..default()
            },
            UiTargetCamera(s_overlay_camera.entity()),
            VideoPlayer::new(
                asset_server.load(spawn_cutscene.0.clone()),
                PlaybackMode::Despawn,
            ),
            Cutscene,
            ChildOf(e_overlay_root_added.entity),
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
