use bevy::prelude::*;
use bevy_enhanced_input::prelude::Action;
use spirit_hotel_movement::components::prelude::{
    CheckMovementIntent, MovementIntent, UpdateMovementIntent,
};
use spirit_hotel_player::components::prelude::Player;
use spirit_hotel_player::inputs::prelude::{WalkHorizontal, WalkVertical};
use std::ops::Deref;

pub fn check_player_input(
    mut commands: Commands,
    s_player: Single<Entity, (With<Player>, With<CheckMovementIntent>)>,
    s_walk_horizontal: Single<&Action<WalkHorizontal>>,
    s_walk_vertical: Single<&Action<WalkVertical>>,
) {
    let walk_horizontal = s_walk_horizontal.into_inner().deref();
    let walk_vertical = s_walk_vertical.into_inner().deref();

    if *walk_horizontal == 0. && *walk_vertical == 0. {
        commands
            .entity(s_player.entity())
            .remove::<(CheckMovementIntent, MovementIntent)>();
        return;
    }

    commands
        .entity(s_player.entity())
        .insert(UpdateMovementIntent)
        .remove::<CheckMovementIntent>();
}
