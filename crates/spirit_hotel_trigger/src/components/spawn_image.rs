use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component, Default)]
pub struct SpawnImage(pub PathBuf);
