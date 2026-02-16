use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Target(pub Entity);