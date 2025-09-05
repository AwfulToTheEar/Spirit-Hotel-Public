use std::path::PathBuf;

use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct LoadLevel(pub PathBuf);
