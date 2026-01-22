use bevy::prelude::*;
use spirit_hotel_core::fades::Fade;
use crate::events::prelude::TriggerActivated;

pub fn fade_in_on_activated_trigger(
    _: On<TriggerActivated>,
    mut next_fade: ResMut<NextState<Fade>>
) {
    next_fade.set(Fade::In);
}