use bevy::prelude::*;
//use bevy::prelude::*;
use crate::components::prelude::MovementDuration;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(MovementDuration)]
pub struct CanMove {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
}
