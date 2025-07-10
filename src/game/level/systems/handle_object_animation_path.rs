use std::path::PathBuf;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledObjectCreated;

use crate::game::animation::{
    components::prelude::AnimationPath, resources::prelude::AnimationAssets,
};

pub fn handle_object_animation_path(
    asset_server: Res<AssetServer>,
    mut animation_assets: ResMut<AnimationAssets>,
    mut e_object_created: EventReader<TiledObjectCreated>,
    q_animation_paths: Query<(Entity, &AnimationPath)>,
) {
    for e in e_object_created.read() {
        let Ok((entity, animation_path)) = q_animation_paths.get(e.entity) else {
            continue;
        };

        let mut path = PathBuf::from("tiled");
        path.push("worlds");
        path.push(animation_path.0.clone());

        animation_assets.0.insert(entity, asset_server.load(path));
    }
}
