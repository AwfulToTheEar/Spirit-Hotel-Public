use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

use crate::game::animation::{
    components::prelude::AnimatableMarker, resources::prelude::AnimationAssets,
};

pub fn init_animatable_entities(
    mut commands: Commands,
    animation_assets: Res<AnimationAssets>,
    q_animatables: Query<(Entity, &Children), With<AnimatableMarker>>,
    q_visuals: Query<Entity, (With<ChildOf>, With<Sprite>, Without<AseAnimation>)>,
) {
    for (entity, children) in q_animatables.iter() {
        for child in children.iter() {
            let Ok(visual_entity) = q_visuals.get(child) else {
                continue;
            };

            let Some(animation_handle) = animation_assets.0.get(&entity) else {
                continue;
            };

            commands.entity(visual_entity).insert(AseAnimation {
                aseprite: animation_handle.clone(),
                ..default()
            });

            break;
        }
    }
}
