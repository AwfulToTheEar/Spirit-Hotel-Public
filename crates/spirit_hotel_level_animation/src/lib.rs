use crate::systems::prelude::{handle_object_animation_path, init_animatable_entities};
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::system_sets::prelude::LevelSystems;

pub mod systems;

pub struct SpiritHotelLevelAnimationPlugin;

impl Plugin for SpiritHotelLevelAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (handle_object_animation_path, init_animatable_entities)
                .after(LevelSystems)
                .in_set(GameplaySystems),
        );
    }
}
