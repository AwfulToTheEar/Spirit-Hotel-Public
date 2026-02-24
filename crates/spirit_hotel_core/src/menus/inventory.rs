use crate::menus::Menu;
use crate::theme::widget;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Inventory), spawn_inventory_menu)
        .add_systems(
            Update,
            go_back.run_if(in_state(Menu::Inventory).and(input_just_pressed(KeyCode::Escape))),
        );
}

#[derive(Component)]
pub struct InventoryMenu;

fn spawn_inventory_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Inventory Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Inventory),
        children![
            widget::header("Inventory"),
            (
                Node {
                    display: Display::Block,
                    ..default()
                },
                InventoryMenu
            )
        ],
    ));
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
