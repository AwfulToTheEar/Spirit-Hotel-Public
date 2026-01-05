use bevy::prelude::*;
use spirit_hotel_core::{
    AppSystems,
    game::{
        GameplaySystems,
    },
};

use crate::systems::prelude::check_trigger_overlap;

pub mod systems;

pub struct SpiritHotelMovementTriggerPlugin;

impl Plugin for SpiritHotelMovementTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_trigger_overlap,
            )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems),
        );
    }
}
