use bevy::prelude::*;
use spirit_hotel_movement::components::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component, Default)]
#[require(SyncMovement, SyncMovementWith<KagumeKagumeDancer>)]
pub struct KagumeKagumeDancer;
