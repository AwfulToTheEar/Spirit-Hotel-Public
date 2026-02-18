use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
pub struct TriggerHeldFor(pub Duration);