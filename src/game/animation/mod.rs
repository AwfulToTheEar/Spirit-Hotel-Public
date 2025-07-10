use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

use crate::{
    AppSystems, PausableSystems, Pause,
    asset_tracking::LoadResource,
    game::{
        GameplaySystems,
        animation::{components::prelude::*, resources::prelude::*, systems::prelude::*},
        level::system_sets::prelude::LevelSystems,
    },
};

pub mod components;
pub mod resources;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        AsepriteUltraPlugin,
        FilterQueryInspectorPlugin::<With<AnimatableMarker>>::default(),
    ))
    .register_type::<AnimationPath>()
    .register_type::<InitAnimation>()
    .load_resource::<AnimationAssets>()
    .add_systems(
        PreUpdate,
        init_animatable_entities
            .after(LevelSystems)
            .in_set(GameplaySystems),
    )
    .add_systems(
        Update,
        (
            start_animation_movement,
            stop_animation_movement,
            start_animation_idle,
            start_init_animations,
            reset_disabled_animations,
        )
            .in_set(AppSystems::Update)
            .in_set(GameplaySystems)
            .in_set(PausableSystems),
    )
    .add_systems(OnEnter(Pause(true)), pause_animation)
    .add_systems(OnEnter(Pause(false)), unpause_animation);
}
