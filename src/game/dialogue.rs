use bevy::{ecs::system::SystemState, prelude::*};
use bevy_yarnspinner::prelude::{DialogueRunner, YarnProject, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;

use crate::{
    AppSystems,
    game::{
        GameplaySystems,
        scene::{
            add_scene_from_dialogue_command, remove_scene_from_dialogue_command,
            toggle_scene_from_dialogue_command,
        },
        trigger::{IsTrigger, TriggerActivatedEvent},
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
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

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct StartConversation(pub String);

pub fn spawn_dialogue_runner(
    world: &mut World,
    params: &mut SystemState<(Res<YarnProject>, Commands)>,
) {
    let (project, mut commands) = params.get_mut(world);
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);

    dialogue_runner
        .commands_mut()
        .add_command(
            "add_game_scene",
            world.register_system(add_scene_from_dialogue_command),
        )
        .add_command(
            "remove_game_scene",
            world.register_system(remove_scene_from_dialogue_command),
        )
        .add_command(
            "toggle_game_scene",
            world.register_system(toggle_scene_from_dialogue_command),
        );

    world.spawn((dialogue_runner, StateScoped(Screen::Gameplay)));
}

pub fn start_conversation_from_activated_trigger(
    mut e_trigger_activated: EventReader<TriggerActivatedEvent>,
    q_conversation_starters: Query<&StartConversation, With<IsTrigger>>,
    mut q_dialogue_runner: Query<&mut DialogueRunner>,
) {
    let Ok(mut dialogue_runner) = q_dialogue_runner.single_mut() else {
        return;
    };

    if dialogue_runner.is_running() {
        return;
    }

    for e in e_trigger_activated.read() {
        let Ok(start_conversation) = q_conversation_starters.get(e.trigger) else {
            continue;
        };

        if !dialogue_runner.node_exists(&start_conversation.0) {
            continue;
        }

        dialogue_runner.start_node(start_conversation.0.clone());

        return;
    }
}
