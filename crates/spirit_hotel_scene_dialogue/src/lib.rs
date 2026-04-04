use crate::systems::observers::prelude::add_scene_dialogue_commands;
use bevy::prelude::*;

pub mod systems;

pub struct SpiritHotelSceneDialoguePlugin;

impl Plugin for SpiritHotelSceneDialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_scene_dialogue_commands);
    }
}
