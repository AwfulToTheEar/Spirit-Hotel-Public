use std::time::Duration;
use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct ContinuousTrigger(pub Option<Duration>);