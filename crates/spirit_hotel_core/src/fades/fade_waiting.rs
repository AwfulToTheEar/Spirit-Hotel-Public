use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use crate::fades::{complete_fade_task, AddFadeTask, ChangeScreen, Fade, FadeData, FadeTasks};
use crate::screens::Screen;
use crate::theme::widget;

pub(super) fn plugin(app: &mut App) {
    app
        .add_observer(complete_fade_task::<ChangeScreen>)
        .add_systems(OnEnter(Fade::Waiting), spawn_fade_waiting)
        .add_systems(Update, fade_out_when_ready.run_if(in_state(Fade::Waiting)))
        .add_fade_task_enter::<ChangeScreen>(change_screen);

}

pub fn spawn_fade_waiting(mut commands: Commands, fade_data: Res<FadeData>) {
    commands.spawn((
        widget::ui_root("Fade Waiting"),
        DespawnOnExit(Fade::Waiting),
        BackgroundColor(fade_data.color),
        RenderLayers::layer(2),
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

pub fn change_screen(
    fade_to_screen: Res<ChangeScreen>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(fade_to_screen.0);
}
