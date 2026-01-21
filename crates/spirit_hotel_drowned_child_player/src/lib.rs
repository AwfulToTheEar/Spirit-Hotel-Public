use crate::systems::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_core::{AppSystems, PausableSystems};
use spirit_hotel_drowned_child::components::prelude::Breathing;

pub mod input;
pub mod systems;

pub struct SpiritHotelDrownedChildPlayerPlugin;

impl Plugin for SpiritHotelDrownedChildPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<Breathing>()
            .add_observer(disable_player_input_action_context)
            .add_observer(enable_player_input_action_context)
            .add_observer(teleport_player_kagume_kagume_start_position)
            .add_systems(
            Update,
            (
                tick_breath_timer
                    .in_set(AppSystems::TickTimers)
                    .in_set(PausableSystems),
                (
                    lose_game_breath_timer_not_held,
                    add_breath_input_actions,
                    remove_breath_input_actions,
                )
                    .in_set(AppSystems::Update),
            )
                .in_set(GameplaySystems),
        );
    }
}
