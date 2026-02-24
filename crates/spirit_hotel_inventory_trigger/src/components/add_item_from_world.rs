use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AddItemFromWorld(pub Entity);