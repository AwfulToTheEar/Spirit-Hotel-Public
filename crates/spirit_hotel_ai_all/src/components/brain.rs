use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum Brain {
    #[default]
    PlayerFollower,
}
