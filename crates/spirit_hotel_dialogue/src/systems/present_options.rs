use crate::components::prelude::{DialogueOption, DialogueUiOptions};
use crate::systems::handle_option_clicked::handle_option_clicked;
use crate::utils::prelude::parse_pretty_text;
use bevy::prelude::*;
use bevy_pretty_text::parser::ParsedPrettyText;
use bevy_pretty_text::prelude::Typewriter;
use bevy_yarnspinner::events::PresentOptions;

pub fn present_options(
    e_options: On<PresentOptions>,
    mut commands: Commands,
    s_dialogue_ui_options: Single<Entity, With<DialogueUiOptions>>,
) {
    commands
        .entity(s_dialogue_ui_options.entity())
        .despawn_children();

    for option in e_options.options.iter() {
        let Ok(spans) = parse_pretty_text(option.line.text.as_str()) else {
            continue;
        };

        commands
            .spawn((
                ChildOf(s_dialogue_ui_options.entity()),
                Typewriter::new(10.0),
                ParsedPrettyText::<Text>::new(spans),
                DialogueOption(option.clone()),
                Button,
            ))
            .observe(handle_option_clicked);
    }
}
