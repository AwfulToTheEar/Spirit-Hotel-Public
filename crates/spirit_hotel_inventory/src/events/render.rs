use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct Render(pub Entity);

impl Render {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}
