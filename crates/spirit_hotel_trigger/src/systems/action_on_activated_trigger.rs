use bevy::prelude::*;
use spirit_hotel_core::dismissibles::DismissiblesOpen;
use spirit_hotel_core::fades::Fade;
use crate::components::prelude::{FromTrigger, IsTrigger};
use crate::events::prelude::TriggerActivated;

pub fn action_on_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut next_fade: ResMut<NextState<Fade>>,
    mut next_dismissibles: ResMut<NextState<DismissiblesOpen>>,
    q_triggers: Query<&FromTrigger, With<IsTrigger>>,
) {
    let Ok(from_trigger) = q_triggers.get(e_trigger_activated.trigger) else {
        return;
    };

    match from_trigger { 
        FromTrigger::FadeIn => next_fade.set(Fade::In),
        FromTrigger::OpenDismissibles => next_dismissibles.set(DismissiblesOpen(true))
    }
}