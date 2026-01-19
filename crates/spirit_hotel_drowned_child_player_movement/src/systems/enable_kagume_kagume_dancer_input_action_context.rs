use bevy::prelude::*;
use bevy_enhanced_input::context::ContextActivity;
use spirit_hotel_drowned_child_movement::components::prelude::KagumeKagumeDancer;

pub fn enable_kagume_kagume_dancer_input_action_context(
    mut commands: Commands,
    s_player: Single<Entity, With<KagumeKagumeDancer>>,
) {
    let entity = s_player.into_inner();

    commands
        .entity(entity)
        .insert(ContextActivity::<KagumeKagumeDancer>::ACTIVE);
}
