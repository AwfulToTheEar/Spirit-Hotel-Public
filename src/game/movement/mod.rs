use bevy::prelude::*;

use crate::game::movement::components::prelude::{CanPush, Pushable};
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
pub(self) mod utils;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CanMove>()
        .register_type::<MovementDuration>()
        .register_type::<CanPush>()
        .register_type::<Pushable>()
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
