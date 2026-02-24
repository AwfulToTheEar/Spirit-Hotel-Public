use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct TriggerActivated {
    pub by: Option<Entity>,
    #[event_target]
    pub trigger: Entity,
}
