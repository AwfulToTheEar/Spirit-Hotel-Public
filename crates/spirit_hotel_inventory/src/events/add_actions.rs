use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct AddActions(pub Entity);

impl AddActions {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}
