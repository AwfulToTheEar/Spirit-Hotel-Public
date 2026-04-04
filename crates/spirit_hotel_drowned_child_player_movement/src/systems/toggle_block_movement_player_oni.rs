use bevy::prelude::*;
use spirit_hotel_drowned_child::components::prelude::KagumeKagume;
use spirit_hotel_movement::components::prelude::MovementFlag;
use spirit_hotel_player::components::prelude::Player;

pub fn toggle_block_movement_player_oni(
    mut commands: Commands,
    s_kagume_kagume: Single<&KagumeKagume>,
    s_player: Single<(Entity, Option<&MovementFlag>), With<Player>>,
) {
    let (entity, option_movement_flag) = s_player.into_inner();

    let kagume_kagume = s_kagume_kagume.into_inner();

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
