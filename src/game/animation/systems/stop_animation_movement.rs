use crate::game::animation::components::prelude::AnimatableMarker;
use crate::game::movement::components::prelude::MoveToLocation;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

pub fn stop_animation_movement(
    mut removed: RemovedComponents<MoveToLocation>,
    q_animateable_entities: Query<&Children, With<AnimatableMarker>>,
    mut q_visuals: Query<&mut AseAnimation>,
) {
    for entity in removed.read() {
        let Ok(children) = q_animateable_entities.get(entity) else {
            continue;
        };

        for child in children.iter() {
            let Ok(mut ase_animation) = q_visuals.get_mut(child) else {
                continue;
            };

            ase_animation.animation.stop();

            break;
        }
    }
}
