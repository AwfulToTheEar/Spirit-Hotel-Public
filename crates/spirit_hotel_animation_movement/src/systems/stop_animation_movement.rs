use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;
use spirit_hotel_animation::components::prelude::AnimatableMarker;
use spirit_hotel_movement::components::prelude::MovementIntent;

pub fn stop_animation_movement(
    mut removed: RemovedComponents<MovementIntent>,
    q_animatable_entities: Query<&Children, With<AnimatableMarker>>,
    mut q_visuals: Query<&mut AseAnimation>,
) {
    for entity in removed.read() {
        let Ok(children) = q_animatable_entities.get(entity) else {
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
