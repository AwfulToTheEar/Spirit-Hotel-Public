use bevy::prelude::*;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
pub enum IsTrigger {
    #[default]
    Instant,
    Interact,
}
