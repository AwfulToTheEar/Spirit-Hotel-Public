use bevy::prelude::*;
use std::path::PathBuf;
use spirit_hotel_trigger::components::prelude::FromTrigger;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(FromTrigger::FadeIn)]
pub struct LoadLevel(pub PathBuf);
