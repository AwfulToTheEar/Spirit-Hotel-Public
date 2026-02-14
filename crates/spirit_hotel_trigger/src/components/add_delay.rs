use bevy::prelude::*;
use crate::components::prelude::FromTrigger;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[require(FromTrigger::FadeIn)]
pub struct AddDelay(pub f32);
