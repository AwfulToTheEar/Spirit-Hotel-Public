//! Spawn the main level.

use std::path::PathBuf;

use avian2d::prelude::{Collider, ColliderDisabled, RigidBody};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;
use bevy_ecs_tiled::prelude::*;

use crate::{
    asset_tracking::LoadResource, audio::music, game::{animation::AnimationPath, movement::CanMove, path_finding::PathNode, trigger::{IsTrigger, TriggerActivatedEvent, TriggerCollider}, GameplaySystems}, screens::Screen, AppSystems, PausableSystems
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

        .add_systems(PreUpdate,(
                handle_object_animation_path,
                handle_object_is_trigger,
                handle_object_path_node,
                handle_collider_static,
            )
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
pub struct LoadLevel(pub PathBuf);

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
    q_maps: Query<(Entity, &TiledMapHandle, &GlobalTransform, &ViewVisibility)>
) {
    for (map_entity, handle, map_transform, view_visibility) in q_maps.iter() {
        if !view_visibility.get() {
            continue;
        }

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

        let mut path = PathBuf::from("tiled");
        path.push("worlds");
        path.push(load_level.0.clone());

        level_assets.world = Some(asset_server.load(path));

        next_screen.set(Screen::Loading);

        return;
    }
}

pub fn handle_object_animation_path(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut e_object_created: EventReader<TiledObjectCreated>,
    q_animations: Query<(Entity, &AnimationPath)>,
) {
    for e in e_object_created.read() {
        let Ok((entity, animation_path)) = q_animations.get(e.entity) else {
            continue;
        };

        let mut path = PathBuf::from("tiled");
        path.push("worlds");
        path.push(animation_path.0.clone());

        commands.entity(entity).insert(AseAnimation {
            aseprite: asset_server.load(path),
            ..default()
        });
    }
}

pub fn handle_object_is_trigger(
    mut commands: Commands,
    mut e_object_created: EventReader<TiledObjectCreated>,
    q_triggers: Query<&Children, With<IsTrigger>>,
    q_colliders: Query<Entity, (With<Collider>, With<ChildOf>)>
) {
    for e in e_object_created.read() {
        let Ok(children) = q_triggers.get(e.entity) else {
            continue;
        };

        for entity in children.iter() {
            if !q_colliders.contains(entity) {
                continue;
            }

            commands.entity(entity).insert(TriggerCollider);
        }
    }
}

pub fn handle_object_path_node(
    mut commands: Commands,
    mut e_object_created: EventReader<TiledObjectCreated>,
    q_path_nodes: Query<&Children, With<PathNode>>,
    q_colliders: Query<Entity, (With<Collider>, With<ChildOf>)>
) {
    for e in e_object_created.read() {
        let Ok(children) = q_path_nodes.get(e.entity) else {
            continue;
        };

        for entity in children.iter() {
            if !q_colliders.contains(entity) {
                continue;
            }

            commands.entity(entity).insert(ColliderDisabled);
        }
    }
}

pub fn handle_collider_static(
    mut commands: Commands,
    mut e_collider_created: EventReader<TiledColliderCreated>,
    q_static: Query<&Children, Without<CanMove>>,
    q_colliders: Query<(Entity, &ChildOf), (With<Collider>, With<ChildOf>)>
) {
    for e in e_collider_created.read() {
        let Ok((entity, child_of)) = q_colliders.get(e.entity) else {
            continue;
        };

        if !q_static.contains(child_of.0) {
            continue;
        }

        commands.entity(entity).insert(RigidBody::Static);
    }
}
