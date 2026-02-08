use bevy::prelude::*;
use crate::components::prelude::CanMove;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(CanMove)]
pub struct MovementAnimations {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
}
