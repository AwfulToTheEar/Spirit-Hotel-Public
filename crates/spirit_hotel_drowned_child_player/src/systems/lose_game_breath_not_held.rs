use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use spirit_hotel_core::screens::Screen;
use spirit_hotel_drowned_child::components::prelude::Breathing;
use spirit_hotel_player::components::prelude::Player;
use crate::input::prelude::HoldBreath;

pub fn lose_game_breath_timer_not_held(
    s_hold_breath: Single<(&ActionEvents, &ActionValue), With<Action<HoldBreath>>>,
    q_other_action_events: Query<&ActionEvents, With<ActionOf<Player>>>,
    q_breathing_player: Query<Entity, (With<Player>, With<Breathing>)>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if q_breathing_player.is_empty() {
        return;
    }

    let (hold_breath_events, hold_breath_value) = s_hold_breath.into_inner();

    if *hold_breath_events == ActionEvents::COMPLETED {
        next_screen.set(Screen::Title);
        return;
    }

    if hold_breath_value.as_bool() {
        return;
    }

    for other_action_events in q_other_action_events.iter() {
        if *other_action_events == ActionEvents::FIRED {
            next_screen.set(Screen::Title);
            return;
        }
    }
}
