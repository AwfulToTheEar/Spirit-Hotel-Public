use bevy::prelude::*;
use crate::components::prelude::FadeInFromTrigger;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[require(FadeInFromTrigger)]
pub struct AddDelay(pub f32);
