use crate::game::drowned_child::components::prelude::{BreathTimer, BreathTimerIndicator};
use crate::screens::Screen;
use crate::theme::widget;
use bevy::prelude::*;

pub fn spawn_breath_timer_ui(
    mut commands: Commands,
    s_breath_timer: Single<&BreathTimer, Added<BreathTimer>>,
) {
    let breath_timer = s_breath_timer.into_inner();

    commands.spawn((
        widget::ui_root("Breath Timer Ui"),
        GlobalZIndex(1),
        StateScoped(Screen::Gameplay),
        children![(
            BreathTimerIndicator,
            widget::header(format!("Breath: {}", breath_timer.0.remaining().as_secs()))
        ),],
    ));
}
