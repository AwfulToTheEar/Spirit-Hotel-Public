use bevy::prelude::*;

#[derive(Message)]
pub struct TriggerEnteredMessage {
    pub entity: Entity,
    pub trigger: Entity,
}
