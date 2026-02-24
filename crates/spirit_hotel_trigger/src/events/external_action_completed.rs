use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct ExternalActionCompleted {
    pub by: Option<Entity>,
    #[event_target]
    pub trigger: Entity
}