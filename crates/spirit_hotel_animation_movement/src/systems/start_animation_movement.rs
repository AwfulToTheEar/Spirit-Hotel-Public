use bevy_aseprite_ultra::prelude::AseAnimation;
use bevy::prelude::*;
use spirit_hotel_movement::components::prelude::{CanMove, MoveToLocation, MovementAnimations, MovementIntent};

pub fn start_animation_movement(
    mut q_moving_entities: Query<(&MovementIntent, &MovementAnimations, &Children), (With<CanMove>, Added<MoveToLocation>)>,
    mut q_visuals: Query<&mut AseAnimation>,
) {
    for (movement_intent, movement_animations, children) in q_moving_entities.iter_mut() {
        for child in children.iter() {
            let Ok(mut ase_animation) = q_visuals.get_mut(child) else {
                continue;
            };

            if movement_intent.0 == Vec2::new(1., 0.) && !movement_animations.right.is_empty() {
                ase_animation.animation.play_loop(&movement_animations.right);
            }

            if movement_intent.0 == Vec2::new(-1., 0.) && !movement_animations.left.is_empty() {
                ase_animation.animation.play_loop(&movement_animations.left);
            }

            if movement_intent.0 == Vec2::new(0., 1.) && !movement_animations.up.is_empty() {
                ase_animation.animation.play_loop(&movement_animations.up);
            }

            if movement_intent.0 == Vec2::new(0., -1.) && !movement_animations.down.is_empty() {
                ase_animation.animation.play_loop(&movement_animations.down);
            }

            break;
        }
    }
}
