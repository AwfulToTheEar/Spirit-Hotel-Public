use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::AppSystems;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_drowned_child::resources::prelude::KagumeKagume;
use spirit_hotel_pathfinding_movement::systems::prelude::apply_intent_from_next_pos;

pub mod systems;

pub struct SpiritHotelDrownedChildPlayerPathfindingMovementPlugin;

impl Plugin for SpiritHotelDrownedChildPlayerPathfindingMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                add_player_kagume_kagume_follow_path.run_if(resource_added::<KagumeKagume>),
                toggle_halt_pathfinding_intent_kagume_kagume_dancers
                    .before(apply_intent_from_next_pos)
                    .run_if(resource_exists::<KagumeKagume>),
                remove_player_kagume_kagume_follow_path.run_if(resource_removed::<KagumeKagume>),
            )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems),
        );
    }
}
