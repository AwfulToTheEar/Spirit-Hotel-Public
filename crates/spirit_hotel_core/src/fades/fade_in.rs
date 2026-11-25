use crate::{AppSystems, fades::{Fade, FadeData}, theme::widget, OverlayCamera};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Fade::In), spawn_fade_in)
        .add_systems(
            Update,
            (
                tick_fade_in.in_set(AppSystems::TickTimers),
                (apply_fade_in, check_fade_in_timer).in_set(AppSystems::Update),
            )
                .run_if(in_state(Fade::In)),
        );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FadeIn(Timer);

pub fn spawn_fade_in(mut commands: Commands, fade_data: Res<FadeData>, s_overlay_camera: Single<Entity, With<OverlayCamera>>) {
    commands.spawn((
        widget::ui_root("Fade In"),
        DespawnOnExit(Fade::In),
        BackgroundColor(fade_data.color.with_alpha(0.)),
        FadeIn(Timer::from_seconds(fade_data.duration, TimerMode::Once)),
        UiTargetCamera(s_overlay_camera.entity())
    ));
}

pub fn tick_fade_in(time: Res<Time>, mut q_fade_in: Query<&mut FadeIn>) {
    let Ok(mut fade_in) = q_fade_in.single_mut() else {
        return;
    };

    fade_in.0.tick(time.delta());
}

pub fn apply_fade_in(mut q_background: Query<(&FadeIn, &mut BackgroundColor)>) {
    for (fade_in, mut background_color) in q_background.iter_mut() {
        background_color.0.set_alpha(fade_in.0.fraction())
    }
}

pub fn check_fade_in_timer(mut next_fade: ResMut<NextState<Fade>>, q_fade_in: Query<&FadeIn>) {
    let Ok(fade_in) = q_fade_in.single() else {
        return;
    };

    if fade_in.0.just_finished() {
        next_fade.set(Fade::Waiting);
    }
}
