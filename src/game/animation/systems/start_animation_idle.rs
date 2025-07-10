use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

pub fn start_animation_idle(q_animateable_entities: Query<&mut AseAnimation>) {
    for mut ase_animation in q_animateable_entities {
        if ase_animation.animation.playing {
            continue;
        }

        ase_animation.animation.play_loop("idle");
    }
}
