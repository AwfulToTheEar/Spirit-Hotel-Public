use crate::systems::prelude::*;
use bevy::prelude::*;

pub mod systems;

pub struct SpiritHotelLevelPathfindingPlugin;

impl Plugin for SpiritHotelLevelPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_object_path_node);
    }
}