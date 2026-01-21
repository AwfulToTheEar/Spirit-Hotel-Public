use crate::components::prelude::*;
use crate::resources::prelude::*;
use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_core::{AppSystems, PausableSystems};

pub mod components;
pub mod resources;
pub mod systems;

pub struct SpiritHotelDrownedChildPlugin;

impl Plugin for SpiritHotelDrownedChildPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Mean>()
            .register_type::<BreathTimer>()
            .register_type::<InitBreathTimer>()
            .register_type::<KagumeKagume>()
            .register_type::<KagumeKagumeStartPathNode>()
            .register_type::<KagumeKagumeStartPosition>()
            .add_systems(
                Update,
                (
                    (
                        update_breath_timer_indicator,
                        update_kagume_kagume_cycle_indicator,
                        remove_kagume_kagume_cycle_timeout,
                        lose_game_breath_timer_timeout,
                    )
                        .in_set(AppSystems::Update)
                        .in_set(PausableSystems),
                    spawn_breath_timer_ui,
                    spawn_kagume_kagume_cycle_ui,
                )
                    .in_set(GameplaySystems),
            );
    }
}
