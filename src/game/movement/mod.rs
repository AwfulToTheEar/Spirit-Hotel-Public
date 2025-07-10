use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    game::{
        GameplaySystems,
        movement::{
            components::prelude::{CanMove, MovementDuration},
            systems::prelude::*,
        },
    },
};

pub mod components;
pub mod systems;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CanMove>()
        .register_type::<MovementDuration>()
        .add_systems(
            Update,
            apply_intent
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems),
        )
        .add_systems(
            FixedUpdate,
            (
                tick_move_to_location.in_set(AppSystems::TickTimers),
                (apply_move_to_location, remove_move_to_location)
                    .chain()
                    .in_set(AppSystems::Update),
            )
                .chain()
                .in_set(GameplaySystems)
                .in_set(PausableSystems),
        );
}
