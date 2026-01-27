use bevy::prelude::*;
use bevy_av1::VideoPlayer;
use spirit_hotel_core::fades::{FadeTaskFinished};
use crate::resources::prelude::FadeToCutscene;

pub fn fade_out_on_cutscene_end(
    _: On<Despawn, VideoPlayer>,
    mut commands: Commands,
) {
    commands.trigger(FadeTaskFinished::<FadeToCutscene>::new());
}