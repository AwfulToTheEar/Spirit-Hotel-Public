use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;

use spirit_hotel_core::{
    screens::Screen,
};

use crate::{
    components::prelude::StartConversation,
    systems::prelude::{spawn_dialogue_runner},
};

pub mod components;
pub mod systems;

pub struct SpiritHotelDialoguePlugin;

impl Plugin for SpiritHotelDialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::new(),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
            .register_type::<StartConversation>()
            .add_systems(
                OnEnter(Screen::Gameplay),
                spawn_dialogue_runner.run_if(resource_exists::<YarnProject>),
            );
    }
}
