use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Component, Reflect, Debug)]
#[reflect(Component, Default)]
pub struct StartCutscene(pub PathBuf);

impl Default for StartCutscene {
    fn default() -> Self {
        Self(PathBuf::from("cutscenes/output.ivf"))
    }
}
