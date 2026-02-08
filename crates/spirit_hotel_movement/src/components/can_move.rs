use bevy::prelude::*;
use crate::components::prelude::MovementDuration;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(MovementDuration)]
pub struct CanMove;
