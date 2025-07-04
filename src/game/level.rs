//! Spawn the main level.

use std::path::PathBuf;

use avian2d::prelude::{Collider, RigidBody};
use bevy::{ecs::entity_disabling::Disabled, prelude::*, render::view::VisibilitySystems};
use bevy_ecs_tiled::prelude::*;

use crate::{
    AppSystems,
    asset_tracking::LoadResource,
    game::{
        GameplaySystems,
        animation::{AnimationAssets, AnimationPath},
        movement::CanMove,
        path_finding::PathNode,
        player::Player,
        trigger::{IsTrigger, TriggerActivatedEvent, TriggerCollider},
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        TiledMapPlugin::default(),
        TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
    ))
    .register_type::<LevelAssets>()
    .register_type::<LoadLevel>()
    .load_resource::<LevelAssets>()
    .configure_sets(PreUpdate, LevelSystems)
    .configure_sets(Update, LevelSystems)
    .configure_sets(PostUpdate, LevelSystems)
    .add_systems(
        PreUpdate,
        (
            handle_object_animation_path,
            handle_object_is_trigger,
            handle_object_path_node,
            handle_collider_static,
            handle_colliders_spawned,
            reparent_player_to_world, //TODO: change to use object created event
        )
            .in_set(LevelSystems)
            .in_set(GameplaySystems),
    )
    .add_systems(
        Update,
        (
            load_level_from_activated_trigger.run_if(resource_exists::<LevelAssets>),
            mark_objects_inside_map,
        )
            .in_set(LevelSystems)
            .in_set(AppSystems::Update)
            .in_set(GameplaySystems),
    )
    .add_systems(
        PostUpdate,
        (
            enable_tiled_entities.before(VisibilitySystems::VisibilityPropagate),
            (
                disable_tiled_entities_when_hidden,
                (
                    disable_tiled_colliders_when_parent_disabled,
                    enable_tiled_colliders_when_parent_enabled,
                ),
            )
                .chain()
                .after(VisibilitySystems::CheckVisibility),
        )
            .in_set(LevelSystems),
    );
}

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LevelSystems;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
    pub world: Option<Handle<TiledWorld>>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            world: None,
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

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct DisabledByVisibility;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct DisabledByParent;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct TiledCollidersSpawned;

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
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

pub fn reparent_player_to_world(
    mut commands: Commands,
    s_world: Single<Entity, With<TiledWorldMarker>>,
    s_player: Single<Entity, Added<Player>>,
) {
    let world = s_world.into_inner();

    let player = s_player.into_inner();

    commands.entity(player).set_parent_in_place(world);
}

pub fn mark_objects_inside_map(
    mut commands: Commands,
    tiled_maps: Res<Assets<TiledMap>>,
    mut q_tiled_map_objects: Query<
        (Entity, &GlobalTransform, Option<&mut InsideMap>),
        (
            Or<(With<TiledMapObject>, With<TiledColliderMarker>)>,
            Changed<GlobalTransform>,
        ),
    >,
    q_maps: Query<(Entity, &TiledMapHandle, &GlobalTransform)>,
) {
    for (map_entity, handle, map_transform) in q_maps.iter() {
        let Some(tiled_map) = tiled_maps.get(handle.0.id()) else {
            continue;
        };

        let map_translation = map_transform.translation();
        let map_size = Vec2::new(tiled_map.rect.max.x as f32, tiled_map.rect.max.y as f32);

        for (entity, transform, inside_map_option) in q_tiled_map_objects.iter_mut() {
            let translation = transform.translation();

            if !(translation.x >= map_translation.x
                && translation.x < map_translation.x + map_size.x
                && translation.y <= map_translation.y
                && translation.y > map_translation.y - map_size.y)
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

            inside_map.0 = map_entity
        }
    }
}

pub fn load_level_from_activated_trigger(
    asset_server: Res<AssetServer>,
    mut level_assets: ResMut<LevelAssets>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut e_trigger_activated: EventReader<TriggerActivatedEvent>,
    q_map_loaders: Query<&LoadLevel, With<IsTrigger>>,
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

pub fn disable_tiled_entities_when_hidden(
    mut commands: Commands,
    q_rendereable_entities: Query<
        (Entity, &Visibility, &InheritedVisibility),
        (
            With<TiledCollidersSpawned>,
            Or<(
                With<TiledMapMarker>,
                With<TiledMapLayer>,
                With<TiledMapObject>,
            )>,
        ),
    >,
) {
    for (entity, visibility, inherited_visibility) in q_rendereable_entities {
        if visibility == Visibility::Visible
            || visibility == Visibility::Inherited && inherited_visibility.get()
        {
            continue;
        }

        commands
            .entity(entity)
            .insert((Disabled, DisabledByVisibility));
    }
}

pub fn enable_tiled_entities(
    mut commands: Commands,
    q_rendereable_entities: Query<
        Entity,
        (
            With<DisabledByVisibility>,
            With<Disabled>,
            Or<(
                With<TiledMapMarker>,
                With<TiledMapLayer>,
                With<TiledMapObject>,
            )>,
        ),
    >,
) {
    for entity in q_rendereable_entities {
        commands
            .entity(entity)
            .remove::<(Disabled, DisabledByVisibility)>();
    }
}

pub fn disable_tiled_colliders_when_parent_disabled(
    mut commands: Commands,
    q_colliders: Query<(Entity, &ChildOf), With<TiledColliderMarker>>,
    q_parents: Query<Entity, With<Disabled>>,
) {
    for (entity, child_of) in q_colliders.iter() {
        if !q_parents.contains(child_of.0) {
            continue;
        };

        commands.entity(entity).insert((DisabledByParent, Disabled));
    }
}

pub fn enable_tiled_colliders_when_parent_enabled(
    mut commands: Commands,
    q_colliders: Query<
        (Entity, &ChildOf),
        (
            With<DisabledByParent>,
            With<Disabled>,
            With<TiledColliderMarker>,
        ),
    >,
    q_parents: Query<Entity>,
) {
    for (entity, child_of) in q_colliders.iter() {
        if !q_parents.contains(child_of.0) {
            continue;
        };

        commands
            .entity(entity)
            .remove::<(DisabledByParent, Disabled)>();
    }
}

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

pub fn handle_object_is_trigger(
    mut commands: Commands,
    mut e_object_created: EventReader<TiledObjectCreated>,
    q_triggers: Query<&Children, With<IsTrigger>>,
    q_colliders: Query<Entity, (With<Collider>, With<ChildOf>)>,
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
    q_colliders: Query<Entity, (With<Collider>, With<ChildOf>)>,
) {
    for e in e_object_created.read() {
        let Ok(children) = q_path_nodes.get(e.entity) else {
            continue;
        };

        for entity in children.iter() {
            if !q_colliders.contains(entity) {
                continue;
            }

            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_collider_static(
    mut commands: Commands,
    mut e_collider_created: EventReader<TiledColliderCreated>,
    q_static: Query<Entity, Without<CanMove>>,
    q_colliders: Query<(Entity, &ChildOf), With<TiledColliderMarker>>,
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

//this breaks if there are no objects with collision or no layers with collision in the world
pub fn handle_colliders_spawned(
    mut commands: Commands,
    e_collider_created: EventReader<TiledColliderCreated>,
    q_tiled_entities: Query<
        Entity,
        (
            Without<TiledCollidersSpawned>,
            Or<(
                With<TiledMapMarker>,
                With<TiledMapLayer>,
                With<TiledMapObject>,
            )>,
        ),
    >,
    mut counter: Local<i16>,
) {
    if e_collider_created.is_empty() {
        return;
    }

    *counter += 1;

    if *counter < 2 {
        return;
    }

    *counter = 0;

    for entity in q_tiled_entities {
        commands.entity(entity).insert(TiledCollidersSpawned);
    }
}
