use bevy::prelude::*;

#[derive(Event)]
pub struct TriggerActivatedEvent {
    pub entity: Entity,
    pub trigger: Entity,
}
