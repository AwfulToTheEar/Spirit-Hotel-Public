use bevy::prelude::*;
use bevy_yarnspinner::prelude::LocalizedLine;

#[derive(Component)]
pub struct DialogueLine(pub LocalizedLine);