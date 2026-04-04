use crate::components::prelude::InventoryItem;
use bevy::prelude::*;
use leafwing_manifest::identifier::Id;

#[derive(Component, Clone)]
pub struct InventoryItemId(pub Id<InventoryItem>);
