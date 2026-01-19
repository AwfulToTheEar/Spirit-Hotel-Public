use crate::systems::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use spirit_hotel_core::AppSystems;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_drowned_child::resources::prelude::KagumeKagume;
use spirit_hotel_drowned_child_movement::components::prelude::KagumeKagumeDancer;

pub mod input;
pub mod systems;

pub struct SpiritHotelDrownedChildPlayerMovementPlugin;

impl Plugin for SpiritHotelDrownedChildPlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<KagumeKagumeDancer>().add_systems(
            Update,
            (
                add_kagume_kagume_dancer_input_actions,
                remove_kagume_kagume_dancer_input_actions,
                add_player_kagume_kagume_dancer.run_if(resource_added::<KagumeKagume>),
                toggle_block_movement_player_oni.run_if(resource_exists::<KagumeKagume>),
            )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems),
        );
    }
}
