use bevy::prelude::*;

#[derive(Component, Clone, PartialEq, Eq, Debug, Reflect)]
#[reflect(Component, PartialEq, Debug, FromWorld, Clone)]
pub struct InsideMap(pub Entity);

impl FromWorld for InsideMap {
    fn from_world(_world: &mut World) -> Self {
        InsideMap(Entity::PLACEHOLDER)
    }
}
