use spirit_hotel_animation::aseprite_ultra::prelude::AseAnimation;
use bevy::prelude::*;
use spirit_hotel_movement::components::prelude::{CanMove, MoveToLocation, MovementIntent};

pub fn start_animation_movement(
    mut q_moving_entities: Query<(&MovementIntent, &CanMove, &Children), Added<MoveToLocation>>,
    mut q_visuals: Query<&mut AseAnimation>,
) {
    for (movement_intent, can_move, children) in q_moving_entities.iter_mut() {
        for child in children.iter() {
            let Ok(mut ase_animation) = q_visuals.get_mut(child) else {
                continue;
            };

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

            break;
        }
    }
}
