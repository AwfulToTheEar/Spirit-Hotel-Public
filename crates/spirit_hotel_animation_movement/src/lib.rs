use crate::systems::prelude::{start_animation_movement, stop_animation_movement};
use bevy::prelude::*;
use spirit_hotel_core::{
    AppSystems, PausableSystems, game::GameplaySystems,
};

pub mod systems;

pub struct SpiritHotelAnimationMovementPlugin;

impl Plugin for SpiritHotelAnimationMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (start_animation_movement, stop_animation_movement)
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems),
        );
    }
}
