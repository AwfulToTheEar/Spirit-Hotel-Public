use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_yarnspinner::events::{
    DialogueCompleteEvent, DialogueStartEvent, PresentLineEvent, PresentOptionsEvent,
};
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;

use spirit_hotel_core::screens::Screen;

use crate::components::prelude::DialogueInputControls;
use crate::{components::prelude::StartConversation, systems::prelude::*};

pub mod components;
pub mod events;
pub mod inputs;
pub mod systems;
pub mod utils;

pub struct SpiritHotelDialoguePlugin;

impl Plugin for SpiritHotelDialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::new(),
            //ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
        .register_type::<StartConversation>()
        .add_input_context::<DialogueInputControls>()
        .add_observer(update_character_name)
        .add_observer(handle_continue_dialogue)
        .add_observer(handle_clear_dialogue_body)
        .add_systems(
            Update,
            (
                show_dialogue_ui.run_if(on_message::<DialogueStartEvent>),
                hide_dialogue_ui.run_if(on_message::<DialogueCompleteEvent>),
                present_line.run_if(on_message::<PresentLineEvent>),
                present_options.run_if(on_message::<PresentOptionsEvent>),
            ),
        )
        .add_systems(
            OnEnter(Screen::Gameplay),
            (
                spawn_dialogue_ui,
                spawn_dialogue_runner.run_if(resource_exists::<YarnProject>),
            ),
        );
    }
}
