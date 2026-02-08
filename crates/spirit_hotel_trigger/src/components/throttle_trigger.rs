use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component, Default)]
pub struct ThrottleTrigger {
    pub throttle: f32,
    pub timer: Option<Timer>
}