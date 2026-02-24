use crate::systems::prelude::handle_selected_item_use;
use bevy::prelude::*;
use spirit_hotel_core::theme::widget;
use spirit_hotel_inventory::events::prelude::AddActions;
use spirit_hotel_player::components::prelude::Player;
use spirit_hotel_trigger::components::prelude::InsideTriggers;

pub fn handle_add_inventory_action_use(
    e_add_actions: On<AddActions>,
    mut commands: Commands,
    _: Single<Entity, (With<InsideTriggers>, With<Player>)>,
) {
    commands.spawn((
        widget::button("Use Item", handle_selected_item_use),
        ChildOf(e_add_actions.0),
    ));
}
