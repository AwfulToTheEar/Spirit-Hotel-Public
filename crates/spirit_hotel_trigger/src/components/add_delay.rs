use crate::components::prelude::FromTrigger;
use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[require(FromTrigger::FadeIn)]
pub struct AddDelay(pub f32);
