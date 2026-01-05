use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy::prelude::*;

use spirit_hotel_core::{
    AppSystems, PausableSystems, Pause, asset_tracking::LoadResource, game::GameplaySystems,
};
use crate::components::prelude::{AnimationPath, InitAnimation};
use crate::resources::prelude::AnimationAssets;
use crate::systems::prelude::{pause_animation, reset_disabled_animations, start_animation_idle, start_init_animations, unpause_animation};

pub mod components;
pub mod resources;
pub mod systems;

pub mod aseprite_ultra {
    pub use bevy_aseprite_ultra::*;
}

pub struct SpiritHotelAnimationPlugin;

impl Plugin for SpiritHotelAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AsepriteUltraPlugin,))
            .register_type::<AnimationPath>()
            .register_type::<InitAnimation>()
            .load_resource::<AnimationAssets>()
            .add_systems(
                Update,
                (
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
}
