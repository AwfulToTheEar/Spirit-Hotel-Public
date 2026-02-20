use bevy::prelude::*;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub enum FromTrigger {
    #[default]
    FadeIn,
    OpenDismissibles
}