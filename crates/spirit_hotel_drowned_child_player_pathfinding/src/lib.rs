use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::AppSystems;
use spirit_hotel_core::game::GameplaySystems;

pub mod systems;

pub struct SpiritHotelDrownedChildPlayerPathfindingPlugin;

impl Plugin for SpiritHotelDrownedChildPlayerPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            lose_game_player_not_at_start_kagume_kagume_cycle_timeout
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems),
        );
    }
}
