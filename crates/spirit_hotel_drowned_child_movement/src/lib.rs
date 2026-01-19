use crate::components::prelude::*;
use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::{AppSystems, PausableSystems};
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_drowned_child::resources::prelude::KagumeKagume;
use spirit_hotel_movement::systems::prelude::{mark_movement_completed, remove_move_to_location_synced};

pub mod components;
pub mod systems;

pub struct SpiritHotelDrownedChildMovementPlugin;

impl Plugin for SpiritHotelDrownedChildMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<KagumeKagumeDancer>()
            .add_systems(
                Update,
                (
                    add_block_movement_kagume_kagume_dancers
                        .run_if(not(resource_exists::<KagumeKagume>)),
                    add_force_movement_kagume_kagume_dancers.run_if(resource_exists::<KagumeKagume>),
                    remove_force_movement_kagume_kagume_dancers
                        .run_if(resource_removed::<KagumeKagume>),
                )
                    .in_set(AppSystems::Update)
                    .in_set(GameplaySystems),
            )
            .add_systems(
                FixedUpdate,
                remove_move_to_location_synced::<KagumeKagumeDancer>
                    .in_set(PausableSystems)
                    .in_set(GameplaySystems)
                    .in_set(AppSystems::Update)
                    .after(mark_movement_completed),
            );
    }
}
