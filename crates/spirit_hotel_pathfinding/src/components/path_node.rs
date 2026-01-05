use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(Transform)]
pub struct PathNode(pub Option<Entity>);
