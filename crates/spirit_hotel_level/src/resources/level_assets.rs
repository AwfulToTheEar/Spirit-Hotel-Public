use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledWorldAsset;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
    pub world: Option<Handle<TiledWorldAsset>>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            world: Some(assets.load("tiled/worlds/world2.world")),
        }
    }
}
