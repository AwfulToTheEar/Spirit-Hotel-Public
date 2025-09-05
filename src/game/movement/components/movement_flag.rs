use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum MovementFlag {
    Force,
    Block,
}
