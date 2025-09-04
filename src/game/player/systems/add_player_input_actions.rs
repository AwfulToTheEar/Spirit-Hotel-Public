use crate::game::player::components::prelude::Player;
use crate::game::player::input::prelude::Walk;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn add_player_input_actions(
    mut commands: Commands,
    s_player: Single<Entity, (With<Player>, Without<Actions<Player>>)>,
) {
    let entity = s_player.into_inner();

    commands.entity(entity).insert(actions!(
        Player[(
            Action::<Walk>::new(),
            Bindings::spawn((Cardinal::wasd_keys(), Cardinal::arrows(),))
        )]
    ));
}
