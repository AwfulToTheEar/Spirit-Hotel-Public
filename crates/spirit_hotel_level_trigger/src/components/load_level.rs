use bevy::prelude::*;
use std::path::PathBuf;
use spirit_hotel_trigger::components::prelude::FadeInFromTrigger;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(FadeInFromTrigger)]
pub struct LoadLevel(pub PathBuf);
