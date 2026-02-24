use spirit_hotel_player::components::prelude::Player;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use spirit_hotel_player::inputs::prelude::WalkHorizontal;
use spirit_hotel_movement::components::prelude::{MoveToLocation, MovementIntent};

pub fn record_player_input_horizontal(
    trigger: On<Fire<WalkHorizontal>>,
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
        .insert(MovementIntent((Vec2::X * trigger.value).normalize()));
}
