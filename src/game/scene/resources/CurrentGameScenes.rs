use bevy::{platform::collections::HashSet, prelude::*};

#[derive(Resource, Default, Clone, Reflect)]
#[reflect(Resource, Default)]
pub struct CurrentGameScenes(pub HashSet<String>);
