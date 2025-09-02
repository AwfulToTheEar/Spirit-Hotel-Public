use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationRepeat, AseAnimation};

use crate::game::animation::components::prelude::InitAnimation;

pub fn start_init_animations(
    mut commands: Commands,
    mut q_animatables: Query<(Entity, &InitAnimation, &Children)>,
    mut q_visuals: Query<&mut AseAnimation>,
) {
    for (entity, init_animation, children) in q_animatables.iter_mut() {
        for child in children.iter() {
            let Ok(mut ase_animation) = q_visuals.get_mut(child) else {
                continue;
            };

            ase_animation.animation.play(
                init_animation.tag.clone(),
                match init_animation.repeat {
                    true => AnimationRepeat::Loop,
                    false => AnimationRepeat::Count(1),
                },
            );

            commands.entity(entity).remove::<InitAnimation>();

            break;
        }
    }
}
