use bevy::math::Vec2;
use bevy_enhanced_input::prelude::*;
#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Walk;
