use crate::game::drowned_child::components::prelude::{BreathTimer, BreathTimerIndicator};
use crate::screens::Screen;
use crate::theme::widget;
use bevy::prelude::*;

pub fn spawn_breath_timer_ui(
    mut commands: Commands,
    q_breath_timer: Query<&BreathTimer, Added<BreathTimer>>,
) {
    if q_breath_timer.is_empty() {
        return;
    }

    commands.spawn((
        widget::ui_root("Breath Timer Ui"),
        GlobalZIndex(1),
        StateScoped(Screen::Gameplay),
        children![(
            BreathTimerIndicator,
            Node {
                align_self: AlignSelf::End,
                ..default()
            },
            widget::header(format!("Hold Breath"))
        )],
    ));
}
