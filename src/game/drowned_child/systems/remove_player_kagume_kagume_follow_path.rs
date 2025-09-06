use crate::game::path_finding::components::prelude::FollowPath;
use crate::game::player::components::prelude::Player;
use bevy::prelude::*;

pub fn remove_player_kagume_kagume_follow_path(
    mut commands: Commands,
    s_player: Single<Entity, With<Player>>,
) {
    let entity = s_player.into_inner();

    commands.entity(entity).remove::<FollowPath>();
}
