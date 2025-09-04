use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    AppSystems,
    game::{
        GameplaySystems,
        player::{components::prelude::Player, systems::prelude::*},
    },
};

pub mod components;
pub mod input;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .add_input_context::<Player>()
        .add_observer(record_player_input)
        .add_systems(
            Update,
            ((apply_player_translate_to_camera, add_player_input_actions)
                .in_set(AppSystems::Update),)
                .in_set(GameplaySystems),
        );
}
