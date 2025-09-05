use bevy::prelude::*;

#[derive(Component)]
pub struct MoveToLocation {
    pub timer: Timer,
    pub start: Vec3,
    pub end: Vec3,
}
