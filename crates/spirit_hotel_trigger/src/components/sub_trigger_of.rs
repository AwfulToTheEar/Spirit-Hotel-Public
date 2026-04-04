use crate::components::prelude::SubTriggers;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = SubTriggers)]
pub struct SubTriggerOf {
    #[relationship]
    pub entity: Entity,
    pub should_be_held: bool,
}
