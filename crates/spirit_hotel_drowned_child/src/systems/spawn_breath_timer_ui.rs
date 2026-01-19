use bevy::prelude::*;
use spirit_hotel_core::screens::Screen;
use spirit_hotel_core::theme::widget;
use crate::components::prelude::{BreathTimer, BreathTimerIndicator};

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
        DespawnOnExit(Screen::Gameplay),
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
