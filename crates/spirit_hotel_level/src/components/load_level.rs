use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct LoadLevel(pub PathBuf);
