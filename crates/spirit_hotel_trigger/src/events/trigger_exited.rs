use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct TriggerExited {
    pub by: Entity,
    #[event_target]
    pub trigger: Entity,
}
