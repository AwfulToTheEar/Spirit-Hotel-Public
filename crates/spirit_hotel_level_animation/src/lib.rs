use crate::systems::prelude::{handle_object_animation_path, init_animatable_entities};
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::system_sets::prelude::LevelSystems;

pub mod systems;

pub struct SpiritHotelLevelAnimationPlugin;

impl Plugin for SpiritHotelLevelAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_object_animation_path).add_systems(
            PreUpdate,
            (init_animatable_entities)
                .in_set(LevelSystems)
                .in_set(GameplaySystems),
        );
    }
}
