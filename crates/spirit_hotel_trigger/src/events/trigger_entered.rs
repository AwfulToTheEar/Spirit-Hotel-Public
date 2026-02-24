use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct TriggerEntered {
    pub by: Entity,
    #[event_target]
    pub trigger: Entity,
}
