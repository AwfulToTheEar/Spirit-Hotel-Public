use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component, Default)]
pub struct InitBreathTimer(pub i32);

impl Default for InitBreathTimer {
    fn default() -> Self {
        InitBreathTimer(30)
    }
}
