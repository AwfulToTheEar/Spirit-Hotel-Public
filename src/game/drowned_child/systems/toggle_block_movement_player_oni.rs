use crate::game::drowned_child::resources::prelude::KagumeKagume;
use crate::game::movement::components::prelude::MovementFlag;
use crate::game::player::components::prelude::Player;
use bevy::prelude::*;

pub fn toggle_block_movement_player_oni(
    mut commands: Commands,
    kagume_kagume: Res<KagumeKagume>,
    s_player: Single<(Entity, Option<&MovementFlag>), With<Player>>,
) {
    let (entity, option_movement_flag) = s_player.into_inner();

    if kagume_kagume.is_oni {
        commands.entity(entity).insert(MovementFlag::Block);
        return;
    }

    let Some(movement_flag) = option_movement_flag else {
        return;
    };

    match movement_flag {
        MovementFlag::Block => {
            commands.entity(entity).remove::<MovementFlag>();
        }
        _ => return,
    }
}
