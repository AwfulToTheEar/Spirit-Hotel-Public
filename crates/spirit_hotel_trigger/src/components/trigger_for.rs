use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TriggerFor(pub Entity);