use crate::components::prelude::Player;
use crate::inputs::prelude::{WalkHorizontal, WalkVertical};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn add_player_input_actions(e_player_added: On<Add, Player>, mut commands: Commands) {
    info!("Adding player input actions.");

    commands.entity(e_player_added.entity).insert(actions!(
        Player[
            (
                Action::<WalkVertical>::new(),
                Bindings::spawn((Bidirectional::new(KeyCode::KeyW, KeyCode::KeyS), Bidirectional::new(KeyCode::ArrowUp, KeyCode::ArrowDown)))
            ),
            (
                Action::<WalkHorizontal>::new(),
                Bindings::spawn((Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA), Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft)))
            )
        ]
    ));
}
