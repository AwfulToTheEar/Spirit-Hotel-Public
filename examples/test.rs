use bevy::input::keyboard::Key::Camera;
use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        EnhancedInputPlugin,
        EguiPlugin::default(),
        WorldInspectorPlugin::new(),
    ))
    .add_input_context::<Player>()
    .add_observer(add_player_input_actions)
    .add_observer(add_player_input_actions_open_inventory)
    .add_observer(|_: On<Start<OpenInventory>>| {
        info!("Open Inventory");
    })
    .add_systems(Startup, |mut commands: Commands| {
        commands.spawn(Camera2d);
        commands.spawn(Player);
    })
    .run();
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct Player;

#[derive(InputAction)]
#[action_output(f32)]
pub struct WalkHorizontal;

#[derive(InputAction)]
#[action_output(f32)]
pub struct WalkVertical;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenInventory;

fn add_player_input_actions(e_player_added: On<Add, Player>, mut commands: Commands) {
    commands.entity(e_player_added.entity).insert(actions!(
        Player[
            (
                Action::<WalkVertical>::new(),
                Bindings::spawn((Bidirectional::new(KeyCode::KeyW, KeyCode::KeyS), Bidirectional::new(KeyCode::ArrowUp, KeyCode::ArrowDown)))
            ),
            (
                Action::<WalkHorizontal>::new(),
                Bindings::spawn((Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA), Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft)))
            )
        ]
    ));
}

fn add_player_input_actions_open_inventory(
    e_general_controls_added: On<Add, Actions<Player>>,
    mut commands: Commands,
) {
    commands.spawn((
        Action::<OpenInventory>::new(),
        ActionOf::<Player>::new(e_general_controls_added.entity),
        bindings![KeyCode::KeyI],
    ));
}
