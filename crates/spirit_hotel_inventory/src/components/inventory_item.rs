use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct InventoryItem {
    pub index: usize,
    pub name: String,
    pub description: String,
}