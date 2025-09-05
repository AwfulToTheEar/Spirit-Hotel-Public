use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementDuration(pub f32);

impl Default for MovementDuration {
    fn default() -> Self {
        Self(0.2)
    }
}
