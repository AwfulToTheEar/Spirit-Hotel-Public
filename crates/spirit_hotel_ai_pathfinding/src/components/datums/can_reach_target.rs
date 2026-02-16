use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use spirit_hotel_ai::Possible;

#[derive(Component, Clone, EnumComponent)]
pub struct CanReachTarget(pub Possible);