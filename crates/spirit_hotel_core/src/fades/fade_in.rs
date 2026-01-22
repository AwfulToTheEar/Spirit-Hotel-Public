use crate::fades::{FadeOutInstantly, FadeToScreen, ReadyForFadeOut};
use crate::{
    AppSystems,
    fades::{Fade, FadeData},
    screens::Screen,
    theme::widget,
};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::OnTransition;
use bevy::prelude::*;
use bevy::reflect::Map;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Fade::In), spawn_fade_in)
        .add_systems(
            Update,
            (
                tick_fade_in.in_set(AppSystems::TickTimers),
                (apply_fade_in, check_fade_in_timer).in_set(AppSystems::Update),
            )
                .run_if(in_state(Fade::In)),
        )
        .add_systems(
            Update,
            fade_out_when_ready
                .run_if(in_state(Fade::Waiting)),
        )
        .add_systems(
            OnEnter(Fade::Waiting),
            fade_out_instantly.run_if(resource_exists::<FadeOutInstantly>),
        )
        .add_systems(
            OnEnter(Fade::Waiting),
            change_screen.run_if(resource_exists::<FadeToScreen>),
        );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FadeIn(Timer);

pub fn spawn_fade_in(mut commands: Commands, fade_data: Res<FadeData>) {
    commands.spawn((
        widget::ui_root("Fade In"),
        DespawnOnExit(Fade::In),
        BackgroundColor(fade_data.color.with_alpha(0.)),
        FadeIn(Timer::from_seconds(fade_data.duration, TimerMode::Once)),
        RenderLayers::layer(1),
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

pub fn fade_out_instantly(mut commands: Commands, mut next_fade: ResMut<NextState<Fade>>) {
    next_fade.set(Fade::Out);

    commands.remove_resource::<FadeOutInstantly>();
}

pub fn fade_out_when_ready(
    mut ready_for_fade_out: ResMut<ReadyForFadeOut>,
    mut next_fade: ResMut<NextState<Fade>>,
) {
    if !ready_for_fade_out.0.is_empty()
        && ready_for_fade_out
            .0
            .iter()
            .find(|&(_, &ready)| !ready)
            .is_some()
    {
        return;
    }

    next_fade.set(Fade::Out);
    ready_for_fade_out.0.clear();
}

pub fn change_screen(
    mut commands: Commands,
    screen: Res<FadeToScreen>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(screen.0);

    commands.remove_resource::<FadeToScreen>();
}
