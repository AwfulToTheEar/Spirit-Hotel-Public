use crate::components::prelude::DialogueOption;
use crate::events::prelude::ClearDialogueBody;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::DialogueRunner;

pub fn handle_option_clicked(
    e_clicked: On<Pointer<Click>>,
    mut commands: Commands,
    q_dialogue_option: Query<&DialogueOption>,
    mut s_dialogue_runner: Single<&mut DialogueRunner>,
) {
    let option = q_dialogue_option.get(e_clicked.entity).unwrap();

    if s_dialogue_runner.select_option(option.0.id).is_err() {
        return;
    };

    commands.trigger(ClearDialogueBody);
}
