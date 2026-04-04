use crate::components::prelude::{
    DialogueInputControls, DialogueUiCharacter, DialogueUiControls, DialogueUiOptions,
    DialogueUiRoot, DialogueUiText,
};
use crate::inputs::prelude::ContinueDialogue;
use bevy::prelude::*;
use bevy_enhanced_input::context::ContextActivity;
use bevy_enhanced_input::prelude::Action;
use bevy_enhanced_input::{actions, bindings};
use spirit_hotel_core::screens::Screen;

pub fn spawn_dialogue_ui(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        DialogueUiRoot,
        DialogueInputControls,
        Name::from("Dialogue Ui"),
        Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_content: AlignContent::End,
            justify_content: JustifyContent::SpaceAround,
            grid_auto_flow: GridAutoFlow::Row,
            grid_template_columns: vec![RepeatedGridTrack::minmax(
                1,
                MinTrackSizingFunction::Auto,
                MaxTrackSizingFunction::Px(800.0 * 0.8),
            )],
            ..default()
        },
        actions!(
            DialogueInputControls[(
                Action::<ContinueDialogue>::new(),
                bindings!(KeyCode::Space, KeyCode::Enter)
            )]
        ),
        ContextActivity::<DialogueInputControls>::INACTIVE,
        children![
            (
                Name::from("Dialogue Character"),
                DialogueUiCharacter,
                Node::default()
            ),
            (
                Name::from("Dialogue Body"),
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                children![
                    (Name::from("Dialogue Text"), DialogueUiText, Node::default()),
                    (
                        Name::from("Dialogue Options"),
                        DialogueUiOptions,
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            ..default()
                        }
                    )
                ],
            ),
            (
                Name::from("Dialogue Controls"),
                DialogueUiControls,
                Node::default()
            )
        ],
        Visibility::Hidden,
    ));
}
