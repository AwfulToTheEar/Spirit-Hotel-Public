use crate::inputs::prelude::OpenInventory;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use spirit_hotel_player::components::prelude::Player;

pub fn add_player_input_actions_open_inventory(
    e_general_controls_added: On<Add, Actions<Player>>,
    mut commands: Commands,
) {
    commands.spawn((
        Action::<OpenInventory>::new(),
        ActionOf::<Player>::new(e_general_controls_added.entity),
        bindings![KeyCode::KeyI],
    ));
}
