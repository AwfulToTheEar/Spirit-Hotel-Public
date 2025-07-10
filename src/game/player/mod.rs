use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    game::{
        GameplaySystems,
        player::{components::prelude::Player, systems::prelude::*},
    },
};

pub mod components;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.register_type::<Player>().add_systems(
        Update,
        (
            record_player_input
                .in_set(AppSystems::RecordInput)
                .in_set(PausableSystems),
            apply_player_translate_to_camera.in_set(AppSystems::Update),
        )
            .in_set(GameplaySystems),
    );
}
