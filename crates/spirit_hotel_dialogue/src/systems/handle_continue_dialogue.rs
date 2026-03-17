use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;
use bevy_yarnspinner::prelude::DialogueRunner;
use crate::events::prelude::ClearDialogueBody;
use crate::inputs::prelude::ContinueDialogue;

pub fn handle_continue_dialogue(
    _: On<Start<ContinueDialogue>>,
    mut commands: Commands,
    mut s_dialogue_runner: Single<&mut DialogueRunner>
) {
    if s_dialogue_runner.is_waiting_for_option_selection() {
        return;
    }

    s_dialogue_runner.continue_in_next_update();
    commands.trigger(ClearDialogueBody)
}