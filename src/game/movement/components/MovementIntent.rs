use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementIntent(pub Vec2);
