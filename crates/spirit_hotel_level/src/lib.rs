use crate::resources::prelude::*;
use crate::system_sets::prelude::*;
use crate::systems::prelude::*;
use avian2d::PhysicsPlugins;
use bevy::camera::visibility::VisibilitySystems;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::regex::RegexSet;
use bevy_ecs_tiled::prelude::{
    TiledFilter, TiledPhysicsAvianBackend, TiledPhysicsPlugin, TiledPlugin, TiledPluginConfig,
};
use spirit_hotel_core::screens::Screen;
use spirit_hotel_core::{AppSystems, asset_tracking::LoadResource, game::GameplaySystems};
use std::env;

pub mod components;
pub mod resources;
pub mod system_sets;
pub mod systems;

pub struct SpiritHotelLevelPlugin;

impl Plugin for SpiritHotelLevelPlugin {
    fn build(&self, app: &mut App) {
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
                        r"^spirit_hotel_.*",
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
        .load_resource::<LevelAssets>()
        .configure_sets(PreUpdate, LevelSystems)
        .configure_sets(Update, LevelSystems)
        .configure_sets(PostUpdate, LevelSystems)
        .add_observer(reparent_entity_to_world)
        .add_systems(OnEnter(Screen::Gameplay), spawn_level)
        .add_systems(
            PreUpdate,
            handle_colliders_spawned
                .in_set(LevelSystems)
                .in_set(GameplaySystems),
        )
        .add_systems(
            Update,
            mark_objects_inside_map
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
}
