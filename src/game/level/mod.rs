use bevy::{prelude::*, render::view::VisibilitySystems};
use bevy_ecs_tiled::{
    TiledMapPlugin,
    prelude::{TiledPhysicsAvianBackend, TiledPhysicsPlugin},
};

use crate::{
    AppSystems,
    asset_tracking::LoadResource,
    game::{
        GameplaySystems,
        level::{
            components::prelude::LoadLevel, resources::prelude::LevelAssets,
            system_sets::prelude::LevelSystems, systems::prelude::*,
        },
    },
};

pub mod components;
pub mod resources;
pub mod system_sets;
pub mod systems;

pub fn plugin(app: &mut App) {
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
