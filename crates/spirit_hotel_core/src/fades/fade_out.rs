use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use crate::{AppSystems, fades::{Fade, FadeData}, theme::widget, Pause};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Fade::Out), spawn_fade_out)
        .add_systems(
            Update,
            (
                tick_fade_out.in_set(AppSystems::TickTimers),
                (apply_fade_out, check_fade_out_timer).in_set(AppSystems::Update),
            )
                .run_if(in_state(Fade::Out)),
        );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FadeOut(Timer);

pub fn spawn_fade_out(mut commands: Commands, fade_data: Res<FadeData>) {
    commands.spawn((
        widget::ui_root("Fade Out"),
        DespawnOnExit(Fade::Out),
        BackgroundColor(fade_data.color.with_alpha(0.)),
        FadeOut(Timer::from_seconds(fade_data.duration, TimerMode::Once)),
        RenderLayers::layer(1)
    ));
}

pub fn tick_fade_out(time: Res<Time>, mut q_fade_out: Query<&mut FadeOut>) {
    let Ok(mut fade_out) = q_fade_out.single_mut() else {
        return;
    };

    fade_out.0.tick(time.delta());
}

pub fn apply_fade_out(mut q_background: Query<(&FadeOut, &mut BackgroundColor)>) {
    for (fade_out, mut background_color) in q_background.iter_mut() {
        background_color
            .0
            .set_alpha(fade_out.0.fraction_remaining())
    }
}

pub fn check_fade_out_timer(mut next_fade: ResMut<NextState<Fade>>, q_fade_out: Query<&FadeOut>) {
    let Ok(fade_out) = q_fade_out.single() else {
        return;
    };

    if !fade_out.0.just_finished() {
        return;
    }

    next_fade.set(Fade::None);
}
