use spirit_hotel_animation::{
    components::prelude::AnimationPath, resources::prelude::AnimationAssets,
};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{ObjectCreated, TiledEvent};
use std::path::PathBuf;

pub fn handle_object_animation_path(
    e_object_created: On<TiledEvent<ObjectCreated>>,
    asset_server: Res<AssetServer>,
    mut animation_assets: ResMut<AnimationAssets>,
    q_animation_paths: Query<&AnimationPath>,
) {
    let Ok(animation_path) = q_animation_paths.get(e_object_created.origin) else {
        return;
    };

    let mut path = PathBuf::from("tiled");
    path.push("worlds");
    path.push(animation_path.0.clone());

    animation_assets.0.insert(e_object_created.origin, asset_server.load(path));
}
