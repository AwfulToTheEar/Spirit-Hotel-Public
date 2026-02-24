use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use spirit_hotel_drowned_child::components::prelude::{BreathTimer, Breathing};
use spirit_hotel_player::components::prelude::Player;

pub fn remove_breath_input_actions(
    mut commands: Commands,
    s_player: Single<Entity, (With<Player>, With<Actions<Breathing>>, With<Breathing>)>,
    _: Populated<&BreathTimer>,
) {
    let entity = s_player.into_inner();
    commands
        .entity(entity)
        .remove_with_requires::<Breathing>()
        .despawn_related::<Actions<Breathing>>();
}
