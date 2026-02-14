//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::fades::ChangeScreen;
use crate::screens::complete_fade_to_screen_task;
use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Title),
        (
            open_main_menu,
            complete_fade_to_screen_task.run_if(resource_exists::<ChangeScreen>),
        ),
    );
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
