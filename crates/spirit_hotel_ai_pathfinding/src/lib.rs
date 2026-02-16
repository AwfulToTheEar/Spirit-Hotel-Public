pub mod components;
pub mod systems;

use crate::components::prelude::{CanReachTarget, CheckIfTargetReachableAction, TargetInReach};
use crate::systems::prelude::handle_check_if_target_reachable_action;
use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_core::{AppSystems, PausableSystems};

pub struct SpiritHotelAiPathfindingPlugin;

impl Plugin for SpiritHotelAiPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_check_if_target_reachable_action
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems),
        );

        register_components!(app, [CanReachTarget, TargetInReach]);
        register_actions!(app, [CheckIfTargetReachableAction]);
    }
}
