use bevy::prelude::*;

#[derive(Resource, Default, Clone, Reflect)]
#[reflect(Resource, Default)]
pub struct StartingGameScene(pub String);
