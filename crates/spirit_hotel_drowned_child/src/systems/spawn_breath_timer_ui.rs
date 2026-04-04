use bevy::prelude::*;
use spirit_hotel_core::screens::Screen;
use spirit_hotel_core::theme::widget;
use crate::components::prelude::{BreathTimer, BreathTimerIndicator};

pub fn spawn_breath_timer_ui(
    mut commands: Commands,
    _: Populated<(), Added<BreathTimer>>,
) {
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
