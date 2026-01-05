use bevy_enhanced_input::prelude::*;
use bevy::prelude::*;

use spirit_hotel_core::{game::GameplaySystems, AppSystems};
use crate::components::prelude::Player;
use crate::systems::prelude::*;

pub mod components;
pub mod input;
pub mod systems;

pub mod enhanced_input {
    pub use bevy_enhanced_input::*;
}

pub struct SpiritHotelPlayerPlugin;

impl Plugin for SpiritHotelPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .register_type::<Player>()
            .add_input_context::<Player>()
            .add_systems(
                Update,
                ((apply_player_translate_to_camera, add_player_input_actions)
                     .in_set(AppSystems::Update),)
                    .in_set(GameplaySystems),
            );
    }
}
