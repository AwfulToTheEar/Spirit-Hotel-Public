use bevy::prelude::*;
use spirit_hotel_core::{AppSystems, PausableSystems};
use spirit_hotel_core::game::GameplaySystems;
use crate::systems::prelude::apply_intent;

pub mod systems;
pub(self) mod utils;

pub struct SpiritHotelPathfindingMovementTriggerPlugin;

impl Plugin for SpiritHotelPathfindingMovementTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_intent
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems),
        );
    }
}
