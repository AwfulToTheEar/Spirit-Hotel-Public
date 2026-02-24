use bevy_enhanced_input::prelude::*;
use bevy::prelude::*;

use spirit_hotel_core::{game::GameplaySystems, AppSystems};
use crate::components::prelude::Player;
use crate::systems::prelude::*;

pub mod components;
pub mod inputs;
pub mod systems;

pub struct SpiritHotelPlayerPlugin;

impl Plugin for SpiritHotelPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_input_context::<Player>()
            .add_observer(add_player_input_actions)
            .add_systems(
                Update,
                ((apply_player_translate_to_camera)
                     .in_set(AppSystems::Update),)
                    .in_set(GameplaySystems),
            );
    }
}
