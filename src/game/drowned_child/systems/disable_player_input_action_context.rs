use crate::game::player::components::prelude::Player;
use bevy::prelude::*;
use bevy_enhanced_input::context::ContextActivity;

pub fn disable_player_input_action_context(
    mut commands: Commands,
    s_player: Single<Entity, With<Player>>,
) {
    let entity = s_player.into_inner();
    commands
        .entity(entity)
        .insert(ContextActivity::<Player>::INACTIVE);
}
