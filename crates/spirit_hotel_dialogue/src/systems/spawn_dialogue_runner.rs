use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnProject;

use spirit_hotel_core::screens::Screen;

pub fn spawn_dialogue_runner(project: Res<YarnProject>, mut commands: Commands) {
    let dialogue_runner = project.create_dialogue_runner(&mut commands);

    commands.spawn((dialogue_runner, DespawnOnExit(Screen::Gameplay)));
}
