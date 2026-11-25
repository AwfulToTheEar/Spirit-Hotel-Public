use crate::{OverlayCamera, OverlayRoot};
use crate::fades::{FadeTask, FadeTaskFinished};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::ui::Node;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(show_image);
}

#[derive(Resource)]
pub struct ShowImage(pub Handle<Image>);

#[derive(Component)]
pub struct ImageOverlay;

pub fn show_image(
    e_overlay_root_added: On<Add, OverlayRoot>,
    mut commands: Commands,
    option_show_image: Option<Res<ShowImage>>,
    option_fade_task: Option<Res<FadeTask<ShowImage>>>,
    s_overlay_camera: Single<Entity, With<OverlayCamera>>
) {
    let Some(show_image) = option_show_image else {
        return;
    };

    commands.spawn((
        Name::new("Image Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        UiTargetCamera(s_overlay_camera.entity()),
        ImageNode::new(show_image.0.clone()),
        ImageOverlay,
        ChildOf(e_overlay_root_added.entity),
    ));

    if option_fade_task.is_none() {
        return;
    }

    commands.trigger(FadeTaskFinished::<ShowImage>::new());
}
