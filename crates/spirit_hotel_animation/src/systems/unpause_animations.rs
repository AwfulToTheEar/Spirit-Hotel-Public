use crate::components::prelude::PausedByState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

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
