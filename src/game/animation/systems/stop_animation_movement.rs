use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

use crate::game::movement::components::prelude::MoveToLocation;

pub fn stop_animation_movement(
    mut removed: RemovedComponents<MoveToLocation>,
    mut q_animateable_entities: Query<&mut AseAnimation>,
) {
    for entity in removed.read() {
        let Ok(mut ase_animation) = q_animateable_entities.get_mut(entity) else {
            continue;
        };

        ase_animation.animation.stop();
    }
}
