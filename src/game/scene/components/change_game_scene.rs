use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct ChangeGameScene {
    pub scene: String,
    pub action: ChangeGameSceneAction,
}

#[derive(Default, Debug, Reflect)]
#[reflect(Default)]
pub enum ChangeGameSceneAction {
    #[default]
    Toggle,
    Add,
    Remove,
}
