use crate::components::prelude::SelectedItemId;
use crate::events::prelude::Render;
use bevy::prelude::*;

pub fn handle_inventory_item_selected(
    e_selected_item_id_inserted: On<Insert, SelectedItemId>,
    mut commands: Commands,
) {
    commands
        .entity(e_selected_item_id_inserted.entity)
        .trigger(Render::new);
}
