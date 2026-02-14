use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource, Default)]
pub struct SpawnCutscene(pub PathBuf);
