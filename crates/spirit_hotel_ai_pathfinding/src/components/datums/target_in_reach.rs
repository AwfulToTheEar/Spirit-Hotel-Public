use bevy::prelude::*;
use bevy_dogoap::prelude::*;

#[derive(Component, Clone, DatumComponent)]
pub struct TargetInReach(pub bool);