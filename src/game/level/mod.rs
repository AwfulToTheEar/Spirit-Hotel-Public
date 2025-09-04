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
use avian2d::PhysicsPlugins;
use bevy::{prelude::*, render::view::VisibilitySystems};
use bevy_ecs_tiled::prelude::{RegexSet, TiledFilter};
use bevy_ecs_tiled::prelude::{TiledPhysicsAvianBackend, TiledPhysicsPlugin};
use bevy_ecs_tiled::tiled::{TiledPlugin, TiledPluginConfig};
use std::env;

pub mod components;
pub mod resources;
pub mod system_sets;
pub mod systems;

pub fn plugin(app: &mut App) {
    let mut path = env::current_dir().unwrap();
    path.push("tiled_types_export.json");

    app.add_plugins((
        TiledPlugin(TiledPluginConfig {
            // Note: if you set this setting to `None`
            // properties won't be exported anymore but
            // you will still be able to load them from the map
            tiled_types_export_file: Some(path),
            tiled_types_filter: TiledFilter::from(
                RegexSet::new([
                    r"^spirit_hotel::.*",
                    r"^bevy_render::view::visibility::Visibility.*",
                    r"^bevy_light_2d::.*",
                ])
                .unwrap(),
            ),
        }),
        PhysicsPlugins::default().with_length_unit(100.0),
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
            handle_world_init_breath_timer,
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
