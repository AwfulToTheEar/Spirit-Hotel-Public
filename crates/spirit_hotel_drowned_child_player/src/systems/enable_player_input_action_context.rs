use bevy::prelude::*;
use bevy_enhanced_input::context::ContextActivity;
use spirit_hotel_player::components::prelude::Player;

pub fn enable_player_input_action_context(
    mut commands: Commands,
    s_player: Single<Entity, With<Player>>,
) {
    let entity = s_player.into_inner();
    commands
        .entity(entity)
        .insert(ContextActivity::<Player>::ACTIVE);
}
