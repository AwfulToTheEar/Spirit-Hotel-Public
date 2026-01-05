use crate::components::prelude::Player;
use crate::input::prelude::{WalkHorizontal, WalkVertical};
use bevy_enhanced_input::prelude::*;
use bevy::prelude::*;

pub fn add_player_input_actions(
    mut commands: Commands,
    s_player: Single<Entity, (With<Player>, Without<Actions<Player>>)>,
) {
    info!("Adding player input actions.");
    let entity = s_player.into_inner();

    commands.entity(entity).insert(actions!(
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
