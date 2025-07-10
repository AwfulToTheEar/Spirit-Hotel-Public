use bevy::prelude::*;

#[derive(Event)]
pub struct TriggerExitedEvent {
    pub entity: Entity,
    pub trigger: Entity,
}
