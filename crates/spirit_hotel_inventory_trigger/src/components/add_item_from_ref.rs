use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AddItemFromRef(pub String);
