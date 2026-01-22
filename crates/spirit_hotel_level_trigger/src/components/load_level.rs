use bevy::prelude::*;
use std::path::PathBuf;
use spirit_hotel_trigger::components::prelude::FadeIn;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(FadeIn)]
pub struct LoadLevel(pub PathBuf);
