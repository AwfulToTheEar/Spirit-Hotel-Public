use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledWorldMarker;

use crate::game::player::components::prelude::Player;

pub fn reparent_player_to_world(
    mut commands: Commands,
    s_world: Single<Entity, With<TiledWorldMarker>>,
    s_player: Single<Entity, Added<Player>>,
) {
    let world = s_world.into_inner();

    let player = s_player.into_inner();

    commands.entity(player).set_parent_in_place(world);
}
