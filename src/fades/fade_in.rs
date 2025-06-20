use bevy::prelude::*;

use crate::{
    fades::{Fade, FadeData}, screens::Screen, theme::widget, AppSystems
};

pub(super) fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(Fade::In), spawn_fade_in)
        .add_systems(
            Update,
            (
                tick_fade_in.in_set(AppSystems::TickTimers),
                (
                    apply_fade_in,
                    check_fade_in_timer
                ).in_set(AppSystems::Update)
            ).run_if(in_state(Fade::In))
        );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FadeIn(Timer);

pub fn spawn_fade_in(
    mut commands: Commands,
    fade_data: Res<FadeData>
) {
    commands.spawn((
        widget::ui_root("Fade In"),
        StateScoped(Fade::In),
        ZIndex(10),
        BackgroundColor(fade_data.color.with_alpha(0.)),
        FadeIn(Timer::from_seconds(fade_data.duration, TimerMode::Once))
    ));
}

pub fn tick_fade_in(
    time: Res<Time>,
    mut q_fade_in: Query<&mut FadeIn>,
) {
    let Ok(mut fade_in) = q_fade_in.single_mut() else {
        return;
    };

    fade_in.0.tick(time.delta());
}

pub fn apply_fade_in(
    mut q_background: Query<(&FadeIn, &mut BackgroundColor)>
) {
    for (fade_in, mut background_color) in q_background.iter_mut() {
        background_color.0.set_alpha(fade_in.0.fraction())
    }
}

pub fn check_fade_in_timer(
    mut fade_data: ResMut<FadeData>,
    mut next_fade: ResMut<NextState<Fade>>,
    mut next_screen: ResMut<NextState<Screen>>,
    q_fade_in: Query<&FadeIn>
) {
    let Ok(fade_in) = q_fade_in.single() else {
        return;
    };

    let Some(screen) = fade_data.next_state else {
        return;
    };

    if fade_in.0.just_finished() {
        fade_data.next_state = None;
        next_fade.set(Fade::None);
        next_screen.set(screen.clone());
    }
}
