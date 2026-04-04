use crate::components::prelude::DialogueUiCharacter;
use crate::events::prelude::SetDialogueCharacter;
use crate::utils::prelude::parse_pretty_text;
use bevy::prelude::*;
use bevy_pretty_text::parser::ParsedPrettyText;

pub fn update_character_name(
    e_set_dialogue_character: On<SetDialogueCharacter>,
    mut commands: Commands,
    s_dialogue_ui_character: Single<Entity, With<DialogueUiCharacter>>,
) {
    if e_set_dialogue_character.0.is_empty() {
        commands
            .entity(s_dialogue_ui_character.entity())
            .despawn_children();
        return;
    }
    let Ok(spans) = parse_pretty_text(e_set_dialogue_character.0.as_str()) else {
        return;
    };

    commands
        .entity(s_dialogue_ui_character.entity())
        .despawn_children()
        .insert(children![(
            Name::from("Dialogue Character Name"),
            ParsedPrettyText::<Text>::new(spans),
            TextFont {
                font_size: 20.,
                ..default()
            }
        )]);
}
