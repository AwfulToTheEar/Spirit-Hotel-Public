use bevy::prelude::*;
use bevy_av1::VideoPlayer;
use spirit_hotel_core::fades::{Fade, ReadyForFadeOut};

pub fn fade_out_on_cutscene_end(
    _: On<Despawn, VideoPlayer>,
    mut ready_for_fade_out: ResMut<ReadyForFadeOut>,
) {
    let Some(cutscene) = ready_for_fade_out.0.get_mut(&"cutscene".to_string()) else {
        return;
    };

    *cutscene = true;
}