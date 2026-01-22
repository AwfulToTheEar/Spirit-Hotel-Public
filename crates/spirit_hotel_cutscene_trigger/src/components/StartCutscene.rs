use bevy::prelude::*;
use std::path::PathBuf;
use spirit_hotel_trigger::components::prelude::FadeIn;

#[derive(Component, Reflect, Debug)]
#[reflect(Component, Default)]
#[require(FadeIn)]
pub struct StartCutscene(pub PathBuf);

impl Default for StartCutscene {
    fn default() -> Self {
        Self(PathBuf::from("cutscenes/output.ivf"))
    }
}
