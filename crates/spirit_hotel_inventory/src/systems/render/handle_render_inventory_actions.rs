use crate::components::prelude::{ItemToCombine, SelectedItemId};
use crate::events::prelude::{AddActions, Render};
use crate::systems::actions::*;
use bevy::prelude::*;
use spirit_hotel_core::menus::inventory::InventoryMenu;
use spirit_hotel_core::theme::widget;

pub fn handle_render_inventory_actions(
    e_render: On<Render>,
    mut commands: Commands,
    s_menu: Single<(&SelectedItemId, Option<&ItemToCombine>), With<InventoryMenu>>,
) {
    let (selected_item, option_item_to_combine) = s_menu.into_inner();

    commands
        .entity(e_render.0)
        .despawn_children()
        .insert((Name::from("Actions"), Node::default()));

    if let Some(item_to_combine) = option_item_to_combine
        && selected_item.0 != item_to_combine.0
    {
        commands.spawn((
            widget::button("Combine", handle_selected_item_combine),
            ChildOf(e_render.0),
        ));
    } else if option_item_to_combine.is_some() {
        commands.spawn((
            widget::button("Cancel", handle_selected_item_combine_with_cancel),
            ChildOf(e_render.0),
        ));
    } else {
        commands.spawn((
            widget::button("Combine With", handle_selected_item_combine_with),
            ChildOf(e_render.0),
        ));
    }

    commands.entity(e_render.0).trigger(AddActions::new);
}
