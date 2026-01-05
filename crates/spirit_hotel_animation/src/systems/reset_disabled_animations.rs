use bevy_aseprite_ultra::prelude::AnimationState;
use bevy::{ecs::entity_disabling::Disabled, prelude::*};

pub fn reset_disabled_animations(mut q_animatables: Query<&mut AnimationState, Added<Disabled>>) {
    for mut animation_state in q_animatables.iter_mut() {
        *animation_state = AnimationState::default();
    }
}
