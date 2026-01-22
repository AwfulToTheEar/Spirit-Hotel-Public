use bevy::prelude::*;

#[derive(Event)]
pub struct TriggerActivated {
    pub entity: Entity,
    pub trigger: Entity,
}
