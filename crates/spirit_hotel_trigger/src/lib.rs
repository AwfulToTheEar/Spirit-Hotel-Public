use bevy::prelude::*;
use spirit_hotel_core::{
    AppSystems,
    game::{
        GameplaySystems
    },
};

use crate::{components::prelude::IsTrigger, messages::prelude::*, systems::prelude::*};

pub mod components;
pub mod messages;
pub mod systems;

pub struct SpiritHotelTriggerPlugin;

impl Plugin for SpiritHotelTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<IsTrigger>()
            .add_message::<TriggerEnteredMessage>()
            .add_message::<TriggerExitedMessage>()
            .add_message::<TriggerActivatedMessage>()
            .add_systems(
                Update,
                (
                    debug_trigger_events,
                    activate_instant_triggers,
                    activate_interact_triggers,
                )
                    .in_set(AppSystems::Update)
                    .in_set(GameplaySystems),
            );
    }
}
