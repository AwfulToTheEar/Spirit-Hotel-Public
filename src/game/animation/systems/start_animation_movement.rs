use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

use crate::game::movement::components::prelude::{CanMove, MoveToLocation, MovementIntent};

pub fn start_animation_movement(
    mut q_moving_entities: Query<
        (&MovementIntent, &mut AseAnimation, &CanMove),
        Added<MoveToLocation>,
    >,
) {
    for (movement_intent, mut ase_animation, can_move) in q_moving_entities.iter_mut() {
        if movement_intent.0 == Vec2::new(1., 0.) && !can_move.right.is_empty() {
            ase_animation.animation.play_loop(&can_move.right);
        }

        if movement_intent.0 == Vec2::new(-1., 0.) && !can_move.left.is_empty() {
            ase_animation.animation.play_loop(&can_move.left);
        }

        if movement_intent.0 == Vec2::new(0., 1.) && !can_move.up.is_empty() {
            ase_animation.animation.play_loop(&can_move.up);
        }

        if movement_intent.0 == Vec2::new(0., -1.) && !can_move.down.is_empty() {
            ase_animation.animation.play_loop(&can_move.down);
        }
    }
}
