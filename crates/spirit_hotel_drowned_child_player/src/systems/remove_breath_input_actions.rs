use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use spirit_hotel_drowned_child::components::prelude::{BreathTimer, Breathing};
use spirit_hotel_player::components::prelude::Player;

pub fn remove_breath_input_actions(
    mut commands: Commands,
    s_player: Single<Entity, (With<Player>, With<Actions<Breathing>>, With<Breathing>)>,
    q_breath_timer: Query<&BreathTimer>,
) {
    if !q_breath_timer.is_empty() {
        return;
    }

    let entity = s_player.into_inner();
    commands
        .entity(entity)
        .remove_with_requires::<Breathing>()
        .despawn_related::<Actions<Breathing>>();
}
