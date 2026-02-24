use bevy::prelude::*;
use leafwing_manifest::identifier::Id;
use crate::components::prelude::InventoryItem;

#[derive(Component, Reflect)]
pub struct ItemToCombine(pub Id<InventoryItem>);