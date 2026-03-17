use crate::components::prelude::{DialogueInputControls, DialogueUiRoot};
use bevy::prelude::*;
use bevy_enhanced_input::context::ContextActivity;

pub fn hide_dialogue_ui(
    mut commands: Commands,
    s_dialogue_ui_root: Single<(Entity, &mut Visibility), (With<DialogueUiRoot>, With<DialogueInputControls>)>,
) {
    let (entity, mut visibility) = s_dialogue_ui_root.into_inner();
    
    *visibility = Visibility::Hidden;

    commands.entity(entity).insert(ContextActivity::<DialogueInputControls>::INACTIVE);
}
