use crate::components::prelude::{KagumeKagumeCycle, KagumeKagumeCycleIndicator};
use bevy::prelude::*;
use spirit_hotel_core::screens::Screen;
use spirit_hotel_core::theme::widget;

pub fn spawn_kagume_kagume_cycle_ui(
    mut commands: Commands,
    _: Populated<&KagumeKagumeCycle, Added<KagumeKagumeCycle>>,
) {
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
