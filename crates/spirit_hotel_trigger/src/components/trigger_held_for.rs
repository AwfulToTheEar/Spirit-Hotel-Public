use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct TriggerHeldFor(pub Duration);
