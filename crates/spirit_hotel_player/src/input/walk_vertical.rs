use bevy::math::Vec2;
use bevy_enhanced_input::prelude::*;

#[derive(InputAction)]
#[action_output(f32)]
pub struct WalkVertical;
