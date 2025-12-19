use bevy::prelude::*;

use crate::game::movement::components::prelude::{MoveToLocation, MovementCompleted};

pub fn apply_move_to_location(
    mut q_moving_transforms: Query<(&mut MoveToLocation, &mut Transform), Without<MovementCompleted>>,
) {
    for (move_to_location, mut transform) in q_moving_transforms.iter_mut() {
        transform.translation = move_to_location
            .start
            .lerp(move_to_location.end, move_to_location.timer.fraction());
    }
}
