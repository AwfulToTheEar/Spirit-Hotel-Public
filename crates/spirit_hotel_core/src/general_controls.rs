use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_input_context::<GeneralControls>()
        .add_systems(Startup, spawn_general_controls);
}

#[derive(Component)]
pub struct GeneralControls;

pub fn spawn_general_controls(mut commands: Commands) {
    commands.spawn((GeneralControls,));
}
