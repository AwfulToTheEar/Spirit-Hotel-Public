use crate::fades::{
    AddFadeTask, ChangeScreen, Delay, Fade, FadeData, FadeTaskFinished, FadeTasks, RegisterFadeTask,
};
use crate::game::ShowImage;
use crate::screens::Screen;
use crate::theme::widget;
use crate::{AppSystems, OverlayCamera, OverlayRoot, Pause};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Fade::Waiting), spawn_fade_waiting)
        .add_systems(Update, fade_out_when_ready.run_if(in_state(Fade::Waiting)))
        .register_fade_task::<Delay>()
        .register_fade_task::<ChangeScreen>()
        .register_fade_task::<ShowImage>()
        .add_fade_task_enter::<ChangeScreen>(change_screen)
        .add_fade_task_update::<Delay>((
            tick_delay
                .in_set(AppSystems::TickTimers)
                .run_if(in_state(Pause(false))),
            complete_delay.in_set(AppSystems::Update),
        ));
}

pub fn spawn_fade_waiting(mut commands: Commands, fade_data: Res<FadeData>, s_overlay_camera: Single<Entity, With<OverlayCamera>>) {
    commands.spawn((
        widget::ui_root("Fade Waiting"),
        DespawnOnExit(Fade::Waiting),
        BackgroundColor(fade_data.color),
        UiTargetCamera(s_overlay_camera.entity()),
        OverlayRoot,
    ));
}

pub fn fade_out_when_ready(
    mut fade_out_tasks: ResMut<FadeTasks>,
    mut next_fade: ResMut<NextState<Fade>>,
) {
    if !fade_out_tasks.0.all() {
        return;
    }

    next_fade.set(Fade::Out);
    fade_out_tasks.0.clear();
}

pub fn tick_delay(time: Res<Time>, mut delay: ResMut<Delay>) {
    delay.0.tick(time.delta());
}

pub fn complete_delay(mut commands: Commands, delay: Res<Delay>) {
    if !delay.0.just_finished() {
        return;
    }

    commands.trigger(FadeTaskFinished::<Delay>::new());
}

pub fn change_screen(
    fade_to_screen: Res<ChangeScreen>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(fade_to_screen.0);
}
