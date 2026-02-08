use bevy::prelude::*;
use crate::components::prelude::SubTriggers;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = SubTriggers)]
pub struct SubTriggerOf(pub Entity);