use bevy::prelude::*;
use crate::components::prelude::SubTriggerOf;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = SubTriggerOf)]
pub struct SubTriggers(Vec<Entity>);