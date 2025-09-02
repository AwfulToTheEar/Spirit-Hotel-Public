use crate::{game::level::resources::prelude::LevelAssets, screens::Screen};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledWorld;
use bevy_ecs_tiled::prelude::{TiledPhysicsAvianBackend, TiledPhysicsSettings, TilemapAnchor};

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
        StateScoped(Screen::Gameplay),
    ));
}
