use bevy::prelude::*;
use spirit_hotel_core::{AppSystems, PausableSystems, game::GameplaySystems};

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
            .add_observer(remove_continuous_trigger_duration)
            .add_observer(mark_triggers_as_activated)
            .add_systems(
                Update,
                (
                    (tick_throttle_trigger_timer, tick_trigger_held_for)
                        .in_set(AppSystems::TickTimers)
                        .in_set(PausableSystems),
                    (
                        activate_interact_triggers,
                        activate_trigger_when_sub_triggers_active,
                        remove_throttle_trigger_timer,
                    )
                        .in_set(AppSystems::Update),
                )
                    .in_set(GameplaySystems),
            );
    }
}
