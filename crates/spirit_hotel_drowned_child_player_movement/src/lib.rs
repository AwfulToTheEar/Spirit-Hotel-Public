use crate::systems::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_core::{AppSystems, PausableSystems};
use spirit_hotel_drowned_child_movement::components::prelude::KagumeKagumeDancer;

pub mod input;
pub mod systems;

pub struct SpiritHotelDrownedChildPlayerMovementPlugin;

impl Plugin for SpiritHotelDrownedChildPlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<KagumeKagumeDancer>()
            .add_observer(add_player_kagume_kagume_dancer)
            .add_systems(
            Update,
            (
                (tick_kagume_kagume_cycle, tick_kagume_kagume_cycle_pause)
                    .in_set(AppSystems::TickTimers)
                    .in_set(PausableSystems),
                (
                    lose_game_player_not_dancing_kagume_kagume_cycle,
                    add_kagume_kagume_dancer_input_actions,
                    remove_kagume_kagume_dancer_input_actions,
                    toggle_block_movement_player_oni,
                )
                    .in_set(AppSystems::Update),
            )
                .in_set(GameplaySystems),
        );
    }
}
