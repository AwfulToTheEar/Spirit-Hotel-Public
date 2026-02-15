use bevy::prelude::*;
use spirit_hotel_movement::components::prelude::CanMove;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(CanMove)]
pub struct MovementAnimations {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
}
