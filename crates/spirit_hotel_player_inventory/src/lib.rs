pub mod inputs;
pub mod systems;

use crate::systems::prelude::{
    add_player_input_actions_open_inventory, add_player_inventory, open_inventory,
};
use bevy::prelude::*;

pub struct SpiritHotelPlayerInventoryPlugin;

impl Plugin for SpiritHotelPlayerInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_player_input_actions_open_inventory)
            .add_observer(open_inventory)
            .add_observer(add_player_inventory);
    }
}
