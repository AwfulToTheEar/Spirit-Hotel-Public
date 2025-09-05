use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GridTileSize(pub UVec2);
