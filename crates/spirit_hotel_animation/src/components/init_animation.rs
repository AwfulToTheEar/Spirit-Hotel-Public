use bevy::prelude::*;
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct InitAnimation {
    pub tag: String,
    pub repeat: bool,
}
