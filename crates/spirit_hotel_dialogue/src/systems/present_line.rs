use crate::components::prelude::{DialogueLine, DialogueUiText};
use crate::events::prelude::SetDialogueCharacter;
use crate::systems::show_options_on_line_typewriter_finish::show_options_on_line_typewriter_finish;
use crate::utils::prelude::parse_pretty_text;
use bevy::prelude::*;
use bevy_pretty_text::parser::{ParsedPrettyText};
use bevy_pretty_text::prelude::{Typewriter};
use bevy_yarnspinner::events::PresentLine;

pub fn present_line(
    mut e_line: On<PresentLine>,
    mut commands: Commands,
    mut s_dialogue_ui_text: Single<Entity, With<DialogueUiText>>,
) {
    
    let name = e_line.line.character_name().unwrap_or("");

    commands.trigger(SetDialogueCharacter(name.to_string()));

    let text = e_line.line.text_without_character_name();

    let Ok(spans) = parse_pretty_text(text.as_str()) else {
        return;
    };

    commands
        .spawn((
            ChildOf(s_dialogue_ui_text.entity()),
            Typewriter::new(10.0),
            ParsedPrettyText::<Text>::new(spans),
            DialogueLine(e_line.line.clone()),
        ))
        .observe(show_options_on_line_typewriter_finish);
    
}
