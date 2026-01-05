use spirit_hotel_player::components::prelude::Player;
use bevy::prelude::*;
use spirit_hotel_player::enhanced_input::prelude::*;
use spirit_hotel_player::input::prelude::WalkVertical;
use spirit_hotel_movement::components::prelude::{MoveToLocation, MovementIntent};

pub fn record_player_input_vertical(
    trigger: On<Fire<WalkVertical>>,
    mut commands: Commands,
    s_player: Single<
        Entity,
        (
            With<Player>,
            Without<MovementIntent>,
            Without<MoveToLocation>,
        ),
    >,
) {
    if trigger.value == 0.0 {
        return;
    }

    let entity = s_player.into_inner();

    commands
        .entity(entity)
        .insert(MovementIntent((Vec2::Y * trigger.value).normalize()));
}
