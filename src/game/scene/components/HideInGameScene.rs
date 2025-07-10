use bevy::prelude::*;

use crate::game::scene::components::prelude::GameSceneObject;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(GameSceneObject)]
pub struct HideInGameScene(pub String);
