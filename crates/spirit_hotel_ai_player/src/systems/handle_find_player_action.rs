use bevy::prelude::*;
use spirit_hotel_ai::components::prelude::{HasTarget, Target};
use spirit_hotel_player::components::prelude::Player;
use crate::components::prelude::FindPlayerAction;

pub fn handle_find_player_action(
    mut commands: Commands,
    mut q_looking_for_player: Populated<(Entity, &mut HasTarget, Option<&mut Target>), With<FindPlayerAction>>,
    q_player: Query<Entity, With<Player>>
) {
    let Ok(player) = q_player.single() else {
        panic!("Expected exactly one player entity");
        return;
    };

    for (entity, mut has_target, mut target) in q_looking_for_player.iter_mut() {
        has_target.0 = true;

        let Some(mut target) = target else {
            commands.entity(entity).insert(Target(player));
            continue;
        };

        target.0 = player;

        commands.entity(entity).remove::<FindPlayerAction>();
    }
}
