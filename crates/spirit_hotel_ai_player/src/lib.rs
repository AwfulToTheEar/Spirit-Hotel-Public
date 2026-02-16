pub mod components;
pub mod systems;

use crate::components::prelude::*;
use crate::systems::prelude::{handle_check_if_player_exists_action, handle_find_player_action};
use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_core::{AppSystems, PausableSystems};

pub struct SpiritHotelAiPlayerPlugin;

impl Plugin for SpiritHotelAiPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_check_if_player_exists_action,
                handle_find_player_action,
            )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems),
        );

        register_actions!(app, [FindPlayerAction, CheckIfPlayerExistsAction]);
        register_components!(app, [PlayerExists]);
    }
}
