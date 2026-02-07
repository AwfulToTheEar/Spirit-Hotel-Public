use bevy::prelude::*;
use spirit_hotel_movement::components::prelude::CanMove;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
#[require(Transform, CanMove)]
pub struct Follow(pub Entity);
