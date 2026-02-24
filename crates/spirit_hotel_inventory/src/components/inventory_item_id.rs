use bevy::prelude::*;
use leafwing_manifest::identifier::Id;
use crate::components::prelude::InventoryItem;

#[derive(Component, Clone)]
pub struct InventoryItemId(pub Id<InventoryItem>);