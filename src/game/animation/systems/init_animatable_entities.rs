use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

use crate::game::animation::{
    components::prelude::AnimatableMarker, resources::prelude::AnimationAssets,
};

pub fn init_animatable_entities(
    mut commands: Commands,
    animation_assets: Res<AnimationAssets>,
    q_animatables: Query<Entity, (With<AnimatableMarker>, Without<AseAnimation>)>,
) {
    for entity in q_animatables.iter() {
        let Some(animation_handle) = animation_assets.0.get(&entity) else {
            continue;
        };

        commands.entity(entity).insert(AseAnimation {
            aseprite: animation_handle.clone(),
            ..default()
        });
    }
}
