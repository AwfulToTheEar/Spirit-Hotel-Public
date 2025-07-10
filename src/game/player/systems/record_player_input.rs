use bevy::prelude::*;

use crate::game::{
    movement::components::prelude::{MoveToLocation, MovementIntent},
    player::components::prelude::Player,
};

pub fn record_player_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<
        Entity,
        (
            With<Player>,
            Without<MovementIntent>,
            Without<MoveToLocation>,
        ),
    >,
) {
    let Ok(entity) = q_player.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction = Vec2::new(1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction = Vec2::new(-1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction = Vec2::new(0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction = Vec2::new(0.0, -1.0);
    }

    if direction == Vec2::ZERO {
        return;
    }

    commands.entity(entity).insert(MovementIntent(direction));
}
