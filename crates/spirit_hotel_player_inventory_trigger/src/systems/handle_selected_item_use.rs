use bevy::prelude::*;
use leafwing_manifest::identifier::Id;
use spirit_hotel_core::menus::Menu;
use spirit_hotel_core::menus::inventory::InventoryMenu;
use spirit_hotel_inventory::components::prelude::{InventoryItem, SelectedItemId};
use spirit_hotel_inventory_trigger::components::prelude::RequiresItem;
use spirit_hotel_player::components::prelude::Player;
use spirit_hotel_trigger::components::prelude::{InsideTriggers, IsTrigger};
use spirit_hotel_trigger::events::prelude::ExternalActionCompleted;

pub fn handle_selected_item_use(
    _: On<Pointer<Click>>,
    mut commands: Commands,
    mut next_menu: ResMut<NextState<Menu>>,
    s_menu: Single<&SelectedItemId, With<InventoryMenu>>,
    s_player: Single<(Entity, &InsideTriggers), With<Player>>,
    q_triggers: Query<&RequiresItem, With<IsTrigger>>,
) {
    let selected_item_id = s_menu.into_inner();

    let (player, inside_triggers) = s_player.into_inner();

    for trigger in inside_triggers.entities.iter() {
        let Ok(requires_item) = q_triggers.get(*trigger) else {
            continue;
        };

        if selected_item_id.0 != Id::<InventoryItem>::from_name(&requires_item.0) {
            continue;
        }

        next_menu.set(Menu::None);

        commands.trigger(ExternalActionCompleted {
            by: Some(player),
            trigger: trigger.clone(),
        });
    }
}
