pub mod systems;
pub mod inputs;

use bevy::prelude::*;
use crate::systems::prelude::{add_player_input_actions_open_inventory, add_player_inventory, open_inventory};

pub struct SpiritHotelPlayerInventoryPlugin;

impl Plugin for SpiritHotelPlayerInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_player_input_actions_open_inventory)
            .add_observer(open_inventory)
            .add_observer(add_player_inventory);
    }
}
