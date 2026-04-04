use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RemoveItemFromRef(pub String);
