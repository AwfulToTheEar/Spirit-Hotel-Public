use bevy::prelude::*;
use spirit_hotel_core::{AppSystems, PausableSystems, game::GameplaySystems};
use crate::{components::prelude::*, systems::prelude::*};

pub mod components;
pub mod systems;

pub struct SpiritHotelMovementPlugin;

impl Plugin for SpiritHotelMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CanMove>()
            .register_type::<MovementDuration>()
            .register_type::<CanPush>()
            .register_type::<Pushable>()
            .add_systems(
                FixedUpdate,
                (
                    tick_move_to_location.in_set(AppSystems::TickTimers),
                    (
                        apply_move_to_location,
                        (remove_move_to_location, mark_movement_completed),
                    )
                        .chain()
                        .in_set(AppSystems::Update),
                )
                    .chain()
                    .in_set(GameplaySystems)
                    .in_set(PausableSystems),
            );
    }
}
