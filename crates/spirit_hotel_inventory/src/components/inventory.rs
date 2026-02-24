use crate::components::inventory_item::InventoryItem;
use bevy::prelude::*;
use leafwing_manifest::identifier::Id;

#[derive(Component)]
pub struct Inventory(pub Vec<Id<InventoryItem>>);

impl Default for Inventory {
    fn default() -> Self {
        Self(vec![Id::from_name("item_1"), Id::from_name("item_2")])
    }
}
