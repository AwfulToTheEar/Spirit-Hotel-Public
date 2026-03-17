use crate::components::prelude::{DialogueOption, DialogueUiOptions, DialogueUiText};
use crate::events::prelude::SetDialogueCharacter;
use crate::utils::prelude::parse_pretty_text;
use bevy::prelude::*;
use bevy_pretty_text::parser::{ParsedPrettyText, TextSpanBundle};
use bevy_pretty_text::prelude::{Typewriter, pretty};
use bevy_yarnspinner::events::{PresentLineEvent, PresentOptionsEvent};
use crate::systems::handle_option_clicked::handle_option_clicked;

pub fn present_options(
    mut options_events: MessageReader<PresentOptionsEvent>,
    mut commands: Commands,
    s_dialogue_ui_options: Single<Entity, With<DialogueUiOptions>>,
) {
    for event in options_events.read() {
        commands
            .entity(s_dialogue_ui_options.entity())
            .despawn_children();

        for option in event.options.iter() {
            let Ok(spans) = parse_pretty_text(option.line.text.as_str()) else {
                continue;
            };

            commands.spawn((
                ChildOf(s_dialogue_ui_options.entity()),
                Typewriter::new(10.0),
                ParsedPrettyText::<Text>::new(spans),
                DialogueOption(option.clone()),
                Button
            )).observe(handle_option_clicked);
        }
    }
}
