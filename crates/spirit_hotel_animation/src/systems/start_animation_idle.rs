use crate::components::prelude::IdleAnimation;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

pub fn start_animation_idle(
    q_animatables: Query<(&IdleAnimation, &Children)>,
    mut q_visuals: Query<&mut AseAnimation>,
) {
    for (idle_animation, children) in q_animatables.iter() {
        for child in children.iter() {
            let Ok(mut ase_animation) = q_visuals.get_mut(child) else {
                continue;
            };

            if ase_animation.animation.playing {
                continue;
            }

            ase_animation.animation.play_loop(idle_animation.0.as_str());
        }
    }
}
