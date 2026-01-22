use bevy::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource, Default)]
pub struct FadeToCutscene(pub PathBuf);

impl Default for FadeToCutscene {
    fn default() -> Self {
        Self(PathBuf::from("cutscenes/output.ivf"))
    }
}
