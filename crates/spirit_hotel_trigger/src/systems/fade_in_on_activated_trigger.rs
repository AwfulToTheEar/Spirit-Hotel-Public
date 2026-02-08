use bevy::prelude::*;
use spirit_hotel_core::fades::Fade;
use crate::components::prelude::{FadeIn, IsTrigger};
use crate::events::prelude::TriggerActivated;

pub fn fade_in_on_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut next_fade: ResMut<NextState<Fade>>,
    q_triggers: Query<Entity, (With<IsTrigger>, With<FadeIn>)>,
) {
    if !q_triggers.contains(e_trigger_activated.trigger) {
        return;
    }

    next_fade.set(Fade::In);
}