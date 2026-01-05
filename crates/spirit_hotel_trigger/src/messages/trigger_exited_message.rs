use bevy::prelude::*;

#[derive(Message)]
pub struct TriggerExitedMessage {
    pub entity: Entity,
    pub trigger: Entity,
}
