use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;

use crate::{
    AppSystems,
    game::{
        GameplaySystems,
        dialogue::{
            components::prelude::StartConversation,
            systems::prelude::{spawn_dialogue_runner, start_conversation_from_activated_trigger},
        },
    },
    screens::Screen,
};

pub mod components;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        YarnSpinnerPlugin::new(),
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ))
    .register_type::<StartConversation>()
    .add_systems(
        OnEnter(Screen::Gameplay),
        spawn_dialogue_runner.run_if(resource_exists::<YarnProject>),
    )
    .add_systems(
        Update,
        start_conversation_from_activated_trigger
            .in_set(AppSystems::Update)
            .in_set(GameplaySystems),
    );
}
