use bevy::prelude::*;

#[derive(Component, Reflect, Copy, Clone)]
#[reflect(Component)]
pub struct MovementIntent(pub Vec2);
