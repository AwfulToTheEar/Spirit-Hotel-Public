use bevy::prelude::*;
use bevy_dogoap::prelude::*;

#[derive(Component, Reflect, Clone, Default, ActionComponent)]
pub struct MoveToTargetAction;