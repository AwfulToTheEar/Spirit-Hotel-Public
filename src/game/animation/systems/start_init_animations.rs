use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationRepeat, AseAnimation};

use crate::game::animation::components::prelude::InitAnimation;

pub fn start_init_animations(
    mut commands: Commands,
    mut q_animatables: Query<(Entity, &InitAnimation, &mut AseAnimation)>,
) {
    for (entity, init_animation, mut ase_animation) in q_animatables.iter_mut() {
        ase_animation.animation.play(
            init_animation.tag.clone(),
            match init_animation.repeat {
                true => AnimationRepeat::Loop,
                false => AnimationRepeat::Count(1),
            },
        );

        commands.entity(entity).remove::<InitAnimation>();
    }
}
