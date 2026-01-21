use bevy::ecs::entity::Entities;
use bevy::prelude::*;
use spirit_hotel_drowned_child::components::prelude::KagumeKagume;
use spirit_hotel_pathfinding_movement::components::prelude::FollowPath;
use spirit_hotel_player::components::prelude::Player;

pub fn remove_player_kagume_kagume_follow_path(
    _: On<Remove, KagumeKagume>,
    mut commands: Commands,
    s_player: Single<Entity, With<Player>>,
) {
    let entity = s_player.into_inner();

    commands.entity(entity).try_remove::<FollowPath>();
}
