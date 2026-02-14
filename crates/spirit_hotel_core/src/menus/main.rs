//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::fades::{Fade, FadeTasks, ChangeScreen, FadeTask};
use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: On<Pointer<Click>>,
    mut commands: Commands,
    resource_handles: Res<ResourceHandles>,
    mut fade_out_tasks: ResMut<FadeTasks>,
    mut next_fade: ResMut<NextState<Fade>>,
) {
    commands.insert_resource(FadeTask::<ChangeScreen>::new(fade_out_tasks.take()));

    if resource_handles.is_all_done() {
        commands.insert_resource(ChangeScreen(Screen::Gameplay));

    } else {
        commands.insert_resource(ChangeScreen(Screen::Loading));
    }

    next_fade.set(Fade::In);
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
