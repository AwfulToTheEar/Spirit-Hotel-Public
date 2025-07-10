use bevy::prelude::*;

use crate::{game::{trigger::{components::prelude::IsTrigger, events::prelude::*, systems::prelude::*}, GameplaySystems}, AppSystems};

pub mod components;
pub mod events;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.register_type::<IsTrigger>()
        .add_event::<TriggerEnteredEvent>()
        .add_event::<TriggerExitedEvent>()
        .add_event::<TriggerActivatedEvent>()
        .add_systems(
            Update,
            (
                check_trigger_overlap,
                debug_trigger_events,
                activate_instant_triggers,
                activate_interact_triggers,
            )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems),
        );
}
