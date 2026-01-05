use crate::components::prelude::PausedByState;
use bevy_aseprite_ultra::prelude::AseAnimation;
use bevy::prelude::*;

pub fn unpause_animation(
    mut commands: Commands,
    q_animateable_entities: Query<(Entity, &mut AseAnimation), With<PausedByState>>,
) {
    for (entity, mut ase_animation) in q_animateable_entities {
        if ase_animation.animation.playing {
            commands.entity(entity).remove::<PausedByState>();
            ase_animation.animation.start();
        }
    }
}
