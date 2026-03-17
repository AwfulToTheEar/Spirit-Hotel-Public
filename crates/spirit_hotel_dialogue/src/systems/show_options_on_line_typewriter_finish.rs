use bevy::prelude::*;
use bevy_pretty_text::prelude::TypewriterFinished;
use bevy_yarnspinner::prelude::DialogueRunner;
use crate::components::prelude::DialogueLine;

pub fn show_options_on_line_typewriter_finish(
    e_typewriter_finished: On<TypewriterFinished>,
    mut s_dialogue_runner: Single<&mut DialogueRunner>,
    q_dialogue_line: Query<&DialogueLine>
) {
    let Ok(line) = q_dialogue_line.get(e_typewriter_finished.entity) else {
        return;
    };
    
    if line.0.is_last_line_before_options() && !s_dialogue_runner.is_waiting_for_option_selection()  {
        s_dialogue_runner.continue_in_next_update();
    }
}