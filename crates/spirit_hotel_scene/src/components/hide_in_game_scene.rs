use bevy::prelude::*;

use crate::components::prelude::GameSceneObject;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(GameSceneObject)]
pub struct HideInGameScene(pub String);
