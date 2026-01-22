use bevy::prelude::*;

#[derive(Event)]
pub struct TriggerExited {
    pub entity: Entity,
    pub trigger: Entity,
}
