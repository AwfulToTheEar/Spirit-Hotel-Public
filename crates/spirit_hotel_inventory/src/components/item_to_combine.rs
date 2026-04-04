use crate::components::prelude::InventoryItem;
use bevy::prelude::*;
use leafwing_manifest::identifier::Id;

#[derive(Component, Reflect)]
pub struct ItemToCombine(pub Id<InventoryItem>);
