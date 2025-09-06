use crate::game::drowned_child::components::prelude::KagumeKagumeDancer;
use bevy::prelude::*;
use bevy_enhanced_input::context::ContextActivity;

pub fn enable_kagume_kagume_dancer_input_action_context(
    mut commands: Commands,
    s_player: Single<Entity, With<KagumeKagumeDancer>>,
) {
    let entity = s_player.into_inner();

    commands
        .entity(entity)
        .insert(ContextActivity::<KagumeKagumeDancer>::ACTIVE);
}
