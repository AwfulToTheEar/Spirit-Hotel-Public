use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    game::{
        GameplaySystems,
        movement::{MoveToLocation, MovementIntent},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>().add_systems(
        Update,
        (
            record_player_input
                .in_set(AppSystems::RecordInput)
                .in_set(PausableSystems),
            apply_player_translate_to_camera.in_set(AppSystems::Update),
        )
            .in_set(GameplaySystems),
    );
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct Player;

pub fn record_player_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<
        Entity,
        (
            With<Player>,
            Without<MovementIntent>,
            Without<MoveToLocation>,
        ),
    >,
) {
    let Ok(entity) = q_player.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction = Vec2::new(1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction = Vec2::new(-1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction = Vec2::new(0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction = Vec2::new(0.0, -1.0);
    }

    if direction == Vec2::ZERO {
        return;
    }

    commands.entity(entity).insert(MovementIntent(direction));
}

pub fn apply_player_translate_to_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    player_option: Option<Single<&GlobalTransform, (With<Player>, Changed<GlobalTransform>)>>,
) {
    let Some(player) = player_option else {
        return;
    };

    let camera_transform = camera.as_mut();

    let player_transform = player.into_inner();

    camera_transform.translation = player_transform.translation();
}
