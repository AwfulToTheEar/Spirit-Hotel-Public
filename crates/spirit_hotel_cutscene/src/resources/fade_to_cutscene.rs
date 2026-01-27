use bevy::prelude::*;
use std::path::{Path, PathBuf};
use spirit_hotel_core::fades::FadeTaskKey;

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource, Default)]
pub struct FadeToCutscene(pub PathBuf);
