use bevy::prelude::*;
use spirit_hotel_core::screens::Screen;
use spirit_hotel_core::theme::widget;
use crate::components::prelude::{KagumeKagumeCycle, KagumeKagumeCycleIndicator};

pub fn spawn_kagume_kagume_cycle_ui(
    mut commands: Commands,
    q_kagume_kagume_cycle: Query<&KagumeKagumeCycle, Added<KagumeKagumeCycle>>,
) {
    if q_kagume_kagume_cycle.is_empty() {
        return;
    }

    commands.spawn((
        widget::ui_root("Kagume Kagume Cycle Ui"),
        GlobalZIndex(1),
        DespawnOnExit(Screen::Gameplay),
        children![(
            KagumeKagumeCycleIndicator,
            Node {
                align_self: AlignSelf::End,
                ..default()
            },
            widget::header(format!("Start Dance"))
        )],
    ));
}
