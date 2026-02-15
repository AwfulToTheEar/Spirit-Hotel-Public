use bevy::prelude::*;
use spirit_hotel_core::{AppSystems, game::GameplaySystems};

use crate::{components::prelude::IsTrigger, systems::prelude::*};

pub mod components;
pub mod events;
pub mod systems;

pub struct SpiritHotelTriggerPlugin;

impl Plugin for SpiritHotelTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<IsTrigger>()
            .add_observer(activate_instant_triggers)
            .add_observer(action_on_activated_trigger)
            .add_observer(add_delay_on_activated_trigger)
            .add_observer(spawn_image_on_activated_trigger)
            .add_systems(
                Update,
                activate_interact_triggers
                    .in_set(AppSystems::Update)
                    .in_set(GameplaySystems),
            );
    }
}
