use crate::components::prelude::{DialogueInputControls, DialogueUiRoot};
use bevy::prelude::*;
use bevy_enhanced_input::context::ContextActivity;
use bevy_yarnspinner::events::DialogueStarted;

pub fn show_dialogue_ui(
    _: On<DialogueStarted>,
    mut commands: Commands,
    s_dialogue_ui_root: Single<
        (Entity, &mut Visibility),
        (With<DialogueUiRoot>, With<DialogueInputControls>),
    >,
) {
    let (entity, mut visibility) = s_dialogue_ui_root.into_inner();

    *visibility = Visibility::Inherited;

    commands
        .entity(entity)
        .insert(ContextActivity::<DialogueInputControls>::ACTIVE);
}
