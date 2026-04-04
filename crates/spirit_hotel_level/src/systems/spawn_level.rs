use crate::resources::prelude::LevelAssets;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{
    TiledPhysicsAvianBackend, TiledPhysicsSettings, TiledWorld, TilemapAnchor,
};
use spirit_hotel_core::screens::Screen;

pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    let Some(world) = &level_assets.world else {
        return;
    };

    commands.spawn((
        Name::new("Level"),
        Visibility::default(),
        TiledWorld(world.clone()),
        //TiledWorldChunking::new(200., 200.),
        //TiledMapHandle(map.clone()),
        TilemapAnchor::Center,
        TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
            backend: TiledPhysicsAvianBackend::Triangulation,
            ..default()
        },
        DespawnOnExit(Screen::Gameplay),
    ));
}
