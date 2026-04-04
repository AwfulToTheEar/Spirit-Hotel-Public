use bevy::prelude::*;
use bevy_northstar::prelude::AgentPos;
use spirit_hotel_core::screens::Screen;
use spirit_hotel_drowned_child::components::prelude::{
    KagumeKagumeCycle, KagumeKagumeStartPosition,
};
use spirit_hotel_player::components::prelude::Player;

pub fn lose_game_player_not_at_start_kagume_kagume_cycle_timeout(
    s_kagume_kagume_cycle: Single<&KagumeKagumeCycle>,
    s_player: Single<&AgentPos, With<Player>>,
    s_kagume_kagume_start_pos: Single<&AgentPos, With<KagumeKagumeStartPosition>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if !s_kagume_kagume_cycle.0.just_finished() {
        return;
    }

    let player_pos = s_player.into_inner();
    let kagume_kagume_start_pos = s_kagume_kagume_start_pos.into_inner();

    if player_pos == kagume_kagume_start_pos {
        return;
    }

    next_screen.set(Screen::Title);
}
