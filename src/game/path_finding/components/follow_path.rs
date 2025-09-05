use bevy::prelude::*;

use crate::game::movement::components::prelude::CanMove;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(Transform, CanMove)]
pub struct FollowPath(pub Option<Entity>);
