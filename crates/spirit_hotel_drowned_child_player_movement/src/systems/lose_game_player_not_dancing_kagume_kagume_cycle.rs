use bevy::prelude::{NextState, ResMut, Single};
use spirit_hotel_core::screens::Screen;
use spirit_hotel_drowned_child::components::prelude::KagumeKagumeCyclePause;

pub fn lose_game_player_not_dancing_kagume_kagume_cycle(
    s_kagume_kagume_cycle_pause: Single<&KagumeKagumeCyclePause>,
    mut next_screen: ResMut<NextState<Screen>>
) {
    if !s_kagume_kagume_cycle_pause.0.just_finished() {
        return;
    }

    next_screen.set(Screen::Title);
}