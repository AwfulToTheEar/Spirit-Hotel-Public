use bevy::prelude::*;
use spirit_hotel_movement::components::prelude::CanMove;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(Transform, CanMove)]
pub struct FollowPath(pub Option<Entity>);
