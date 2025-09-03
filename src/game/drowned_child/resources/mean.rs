use bevy::prelude::*;

#[derive(Resource, Clone, Debug, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct Mean(pub i32);
