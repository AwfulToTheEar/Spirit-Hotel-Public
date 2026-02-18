use bevy::prelude::*;

#[derive(Event)]
pub struct TriggerActivated {
    pub entity: Option<Entity>,
    pub trigger: Entity,
}
