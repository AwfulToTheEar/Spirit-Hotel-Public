use crate::game::drowned_child::components::prelude::KagumeKagumeDancer;
use crate::game::player::components::prelude::Player;
use bevy::prelude::*;

pub fn add_player_kagume_kagume_dancer(
    mut commands: Commands,
    s_player: Single<Entity, With<Player>>,
) {
    let entity = s_player.into_inner();

    commands.entity(entity).insert(KagumeKagumeDancer);
}
