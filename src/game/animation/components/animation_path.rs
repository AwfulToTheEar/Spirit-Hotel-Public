use std::path::PathBuf;

use bevy::prelude::*;

use crate::game::animation::components::animatable_marker::AnimatableMarker;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(AnimatableMarker)]
pub struct AnimationPath(pub PathBuf);
