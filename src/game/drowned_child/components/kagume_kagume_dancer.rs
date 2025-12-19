use bevy::prelude::*;

use crate::game::movement::components::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component, Default)]
#[require(SyncMovement, SyncMovementWith<KagumeKagumeDancer>)]
pub struct KagumeKagumeDancer;
