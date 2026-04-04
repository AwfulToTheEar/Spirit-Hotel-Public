use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
pub struct InventoryItemRef(pub String);
