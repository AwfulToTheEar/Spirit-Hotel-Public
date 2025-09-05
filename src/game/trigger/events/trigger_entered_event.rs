use bevy::prelude::*;

#[derive(Event)]
pub struct TriggerEnteredEvent {
    pub entity: Entity,
    pub trigger: Entity,
}
