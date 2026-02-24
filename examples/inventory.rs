use bevy::prelude::*;
use bevy_enhanced_input::context::InputContextAppExt;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_enhanced_input::prelude::Start;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_manifest::asset_state::SimpleAssetState;
use leafwing_manifest::plugin::ManifestPlugin;
use spirit_hotel_core::menus::inventory::InventoryMenu;
use spirit_hotel_core::SpiritHotelCorePlugin;
use spirit_hotel_core::screens::Screen;
use spirit_hotel_inventory::SpiritHotelInventoryPlugin;
use spirit_hotel_player::SpiritHotelPlayerPlugin;
use spirit_hotel_player::components::prelude::Player;
use spirit_hotel_player::systems::prelude::add_player_input_actions;
use spirit_hotel_player_inventory::inputs::prelude::OpenInventory;
use spirit_hotel_player_inventory::SpiritHotelPlayerInventoryPlugin;
use spirit_hotel_player_inventory::systems::prelude::{add_player_input_actions_open_inventory, add_player_inventory};

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .init_state::<SimpleAssetState>()
        .add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            EnhancedInputPlugin,
            ManifestPlugin::<SimpleAssetState>::default(),
            SpiritHotelInventoryPlugin,
        ))
        .add_input_context::<Player>()
        .add_observer(add_player_input_actions)
        .add_observer(add_player_input_actions_open_inventory)
        .add_observer(add_player_inventory)
        .add_observer(|_: On<Start<OpenInventory>>, mut commands: Commands| {
            info!("Open Inventory");
            commands.spawn((InventoryMenu, Node::default()));
        })
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
            commands.spawn(Player);
        })
        .run();
}
