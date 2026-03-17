use crate::components::prelude::{DialogueLine, DialogueUiText};
use crate::events::prelude::SetDialogueCharacter;
use crate::systems::show_options_on_line_typewriter_finish::show_options_on_line_typewriter_finish;
use crate::utils::prelude::parse_pretty_text;
use bevy::prelude::*;
use bevy_pretty_text::parser::{ParsedPrettyText, TextSpanBundle};
use bevy_pretty_text::prelude::{Typewriter, pretty};
use bevy_yarnspinner::events::PresentLineEvent;
use bevy_yarnspinner::prelude::DialogueRunner;

pub fn present_line(
    mut line_events: MessageReader<PresentLineEvent>,
    mut commands: Commands,
    mut s_dialogue_ui_text: Single<Entity, With<DialogueUiText>>,
) {
    for event in line_events.read() {
        let name = event.line.character_name().unwrap_or("");

        commands.trigger(SetDialogueCharacter(name.to_string()));

        let text = event.line.text_without_character_name();

        let Ok(spans) = parse_pretty_text(text.as_str()) else {
            continue;
        };

        commands
            .spawn((
                ChildOf(s_dialogue_ui_text.entity()),
                Typewriter::new(10.0),
                ParsedPrettyText::<Text>::new(spans),
                DialogueLine(event.line.clone()),
            ))
            .observe(show_options_on_line_typewriter_finish);
    }
}
