use crate::game::player::input::prelude::Walk;
use crate::game::{
    movement::components::prelude::{MoveToLocation, MovementIntent},
    player::components::prelude::Player,
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn record_player_input(
    trigger: Trigger<Fired<Walk>>,
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
    if trigger.value == Vec2::ZERO {
        return;
    }

    let entity = s_player.into_inner();

    commands
        .entity(entity)
        .insert(MovementIntent(trigger.value));
}
