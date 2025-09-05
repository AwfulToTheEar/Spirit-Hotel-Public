use crate::game::drowned_child::resources::prelude::KagumeKagume;
use crate::game::movement::components::prelude::MovementFlag;
use crate::game::player::components::prelude::Player;
use bevy::prelude::*;

pub fn toggle_block_movement_player_oni(
    mut commands: Commands,
    kagume_kagume: Res<KagumeKagume>,
    s_player: Single<Entity, With<Player>>,
) {
    let entity = s_player.into_inner();

    if kagume_kagume.is_oni {
        commands.entity(entity).insert(MovementFlag::Block);
        return;
    }

    commands.entity(entity).remove::<MovementFlag>();
}
