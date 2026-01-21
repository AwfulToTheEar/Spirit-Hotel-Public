use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::AppSystems;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_pathfinding_movement::systems::prelude::apply_intent_from_next_pos;

pub mod systems;

pub struct SpiritHotelDrownedChildPlayerPathfindingMovementPlugin;

impl Plugin for SpiritHotelDrownedChildPlayerPathfindingMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_player_kagume_kagume_follow_path)
            .add_observer(remove_player_kagume_kagume_follow_path)
            .add_systems(
                Update,
                toggle_halt_pathfinding_intent_kagume_kagume_dancers
                    .before(apply_intent_from_next_pos)
                    .in_set(AppSystems::Update)
                    .in_set(GameplaySystems),
            );
    }
}
