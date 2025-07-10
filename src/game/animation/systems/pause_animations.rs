use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

use crate::game::animation::components::prelude::PausedByState;

pub fn pause_animation(
    mut commands: Commands,
    q_animateable_entities: Query<(Entity, &mut AseAnimation)>,
) {
    for (entity, mut ase_animation) in q_animateable_entities {
        if ase_animation.animation.playing {
            commands.entity(entity).insert(PausedByState);
            ase_animation.animation.pause();
        }
    }
}
