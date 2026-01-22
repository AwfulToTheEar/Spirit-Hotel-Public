use bevy::prelude::*;

#[derive(Event)]
pub struct TriggerEntered {
    pub entity: Entity,
    pub trigger: Entity,
}
