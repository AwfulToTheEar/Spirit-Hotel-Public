use bevy::prelude::*;
use leafwing_manifest::identifier::Id;
use crate::components::inventory_item::InventoryItem;

#[derive(Component, Reflect)]
pub struct SelectedItemId(pub Id<InventoryItem>);