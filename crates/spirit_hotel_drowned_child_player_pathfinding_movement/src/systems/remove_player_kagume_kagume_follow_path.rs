use bevy::prelude::*;
use spirit_hotel_pathfinding_movement::components::prelude::FollowPath;
use spirit_hotel_player::components::prelude::Player;

pub fn remove_player_kagume_kagume_follow_path(
    mut commands: Commands,
    s_player: Single<Entity, With<Player>>,
) {
    let entity = s_player.into_inner();

    commands.entity(entity).remove::<FollowPath>();
}
