use bevy::prelude::*;

#[derive(Message)]
pub struct TriggerActivatedMessage {
    pub entity: Entity,
    pub trigger: Entity,
}
