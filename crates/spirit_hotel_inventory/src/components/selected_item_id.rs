use crate::components::inventory_item::InventoryItem;
use bevy::prelude::*;
use leafwing_manifest::identifier::Id;

#[derive(Component, Reflect)]
pub struct SelectedItemId(pub Id<InventoryItem>);
