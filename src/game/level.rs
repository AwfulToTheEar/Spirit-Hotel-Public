//! Spawn the main level.

use avian2d::prelude::Collider;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;
use bevy_ecs_tiled::prelude::*;

use crate::{
    asset_tracking::LoadResource, audio::music, game::{animation::AnimationPath, trigger::{IsTrigger, TriggerActivatedEvent, TriggerCollider}, GameplaySystems}, screens::Screen, AppSystems, PausableSystems
};

pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins((
            TiledMapPlugin::default(),
            TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
        ))

        .register_type::<LevelAssets>()
        .register_type::<LoadLevel>()

        .load_resource::<LevelAssets>()

        .add_systems(PreUpdate,
            handle_object_event
                .in_set(GameplaySystems)
        )

        .add_systems(Update,
            (
                    load_level_from_activated_trigger.run_if(resource_exists::<LevelAssets>),
                    mark_objects_inside_map
                )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems)
        );
}


#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
    pub world: Option<Handle<TiledWorld>>
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            world: None
        }
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct LoadLevel(pub String);

#[derive(Component, Clone, PartialEq, Eq, Debug, Reflect)]
#[reflect(Component, PartialEq, Debug, FromWorld, Clone)]
pub struct InsideMap(pub Entity);

impl FromWorld for InsideMap {
    fn from_world(_world: &mut World) -> Self {
        InsideMap(Entity::PLACEHOLDER)
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    let Some(world) = &level_assets.world else {
        return;
    };

    commands.spawn((
        Name::new("Level"),
        Visibility::default(),
        TiledWorldHandle(world.clone()),
        //TiledWorldChunking::new(200., 200.),
        //TiledMapHandle(map.clone()),
        TilemapAnchor::Center,
        TiledPhysicsSettings::<TiledPhysicsAvianBackend>::default(),
        StateScoped(Screen::Gameplay),
    ));
}

pub fn mark_objects_inside_map(
    mut commands: Commands,
    tiled_maps: Res<Assets<TiledMap>>,
    mut q_tiled_map_objects: Query<(Entity, &GlobalTransform, Option<&mut InsideMap>), (With<TiledMapObject>, Changed<GlobalTransform>)>,
    q_maps: Query<(Entity, &TiledMapHandle, &GlobalTransform)>
) {
    for (map_entity, handle, map_transform) in q_maps.iter() {
        let Some(tiled_map) = tiled_maps.get(handle.0.id()) else {
            continue;
        };

        let map_translation = map_transform.translation();
        let map_size = Vec2::new(tiled_map.rect.max.x as f32, tiled_map.rect.max.y as f32);

        for (entity, transform, inside_map_option) in q_tiled_map_objects.iter_mut() {
            let translation = transform.translation();

            if
                !(translation.x >= map_translation.x &&
                translation.x < map_translation.x + map_size.x &&
                translation.y <= map_translation.y &&
                translation.y > map_translation.y - map_size.y)
            {
                continue;
            }

            let Some(mut inside_map) = inside_map_option else {
                commands.entity(entity).insert(InsideMap(map_entity));
                continue;
            };

            if inside_map.0 == map_entity {
                continue;
            }

            info!("map changed to {}", map_entity);

            inside_map.0 = map_entity
        }
    }
}

pub fn load_level_from_activated_trigger(
    asset_server: Res<AssetServer>,
    mut level_assets: ResMut<LevelAssets>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut e_trigger_activated: EventReader<TriggerActivatedEvent>,
    q_map_loaders: Query<&LoadLevel, With<IsTrigger>>
) {
    for e in e_trigger_activated.read() {
        let Ok(load_level) = q_map_loaders.get(e.trigger) else {
            continue;
        };

        level_assets.world = Some(asset_server.load(load_level.0.clone()));

        next_screen.set(Screen::Loading);

        return;
    }
}

pub fn handle_object_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut e_object_created: EventReader<TiledObjectCreated>,
    q_triggers: Query<&Children, With<IsTrigger>>,
    q_animations: Query<(Entity, &AnimationPath)>,
    q_colliders: Query<Entity, (With<Collider>, With<ChildOf>)>
) {
    for e in e_object_created.read() {
        if let Ok(children) = q_triggers.get(e.entity) {
            for entity in children.iter() {
                if !q_colliders.contains(entity) {
                    continue;
                }

                commands.entity(entity).insert(TriggerCollider);
            }
        }

        if let Ok((entity, animation_path)) = q_animations.get(e.entity) {
            commands.entity(entity).insert(AseAnimation {
                aseprite: asset_server.load(animation_path.0.clone()),
                ..default()
            });
        }
    }
}
