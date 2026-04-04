use bevy::prelude::*;
use spirit_hotel_trigger::components::prelude::FromTrigger;
use std::path::PathBuf;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(FromTrigger::FadeIn)]
pub struct LoadLevel(pub PathBuf);
