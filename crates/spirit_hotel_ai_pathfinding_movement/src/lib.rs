pub mod components;
pub mod systems;

use crate::components::prelude::MoveToTargetAction;
use crate::systems::{
    handle_move_to_target_action_start_following, handle_move_to_target_action_target_reached,
};
use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_core::{AppSystems, PausableSystems};

pub struct SpiritHotelAiPathfindingMovementPlugin;

impl Plugin for SpiritHotelAiPathfindingMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_move_to_target_action_start_following,
                handle_move_to_target_action_target_reached,
            )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems),
        );

        register_actions!(app, [MoveToTargetAction]);
    }
}
