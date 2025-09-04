use crate::game::drowned_child::components::prelude::BreathTimer;
use crate::screens::Screen;
use bevy::prelude::*;

pub fn lose_game_breath_timer_timeout(
    s_breath_timer: Single<&BreathTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let breath_timer = s_breath_timer.into_inner();

    if breath_timer.0.just_finished() {
        next_screen.set(Screen::Title)
    }
}
