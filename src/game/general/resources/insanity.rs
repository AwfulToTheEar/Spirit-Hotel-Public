use bevy::prelude::*;

#[derive(Resource, Clone, Debug, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct Insanity(pub i32);
