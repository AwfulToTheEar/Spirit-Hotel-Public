use bevy::prelude::*;
use crate::components::prelude::{DialogueUiOptions, DialogueUiText};
use crate::events::prelude::ClearDialogueBody;

pub fn handle_clear_dialogue_body(
    _: On<ClearDialogueBody>,
    mut commands: Commands,
    s_dialogue_text: Single<Entity, With<DialogueUiText>>,
    s_dialogue_options: Single<Entity, With<DialogueUiOptions>>
) {
    commands.entity(s_dialogue_text.entity()).despawn_children();
    commands.entity(s_dialogue_options.entity()).despawn_children();   
}