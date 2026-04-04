use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct InventoryMenuSections(pub HashMap<InventoryMenuSection, Entity>);

impl Default for InventoryMenuSections {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl InventoryMenuSections {
    pub fn get(&self, section: InventoryMenuSection) -> Option<&Entity> {
        self.0.get(&section)
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum InventoryMenuSection {
    SelectedInventoryItem,
    InventoryItems,
    Actions,
}
