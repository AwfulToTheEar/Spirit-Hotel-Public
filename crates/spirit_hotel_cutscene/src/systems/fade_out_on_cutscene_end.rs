use bevy::prelude::*;
use bevy_av1::VideoPlayer;
use spirit_hotel_core::fades::Fade;

pub fn fade_out_on_cutscene_end(
    _: On<Despawn, VideoPlayer>,
    mut next_fade: ResMut<NextState<Fade>>
) {
    next_fade.set(Fade::Out)
}