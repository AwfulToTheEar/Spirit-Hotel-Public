use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;

use crate::{game::{movement::{CanMove, MoveToLocation, MovementIntent}, GameplaySystems}, AppSystems, PausableSystems, Pause};

pub(super) fn plugin(app: &mut App) {
    app
        .register_type::<AnimationPath>()

        .add_systems(Update,
            (
                start_animation_movement,
                stop_animation_movement,
                start_animation_idle
            )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems)
        )
        .add_systems(OnEnter(Pause(true)), pause_animation)
        .add_systems(OnEnter(Pause(false)), unpause_animation);

}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct PausedByState;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct AnimationPath(pub String);

pub fn start_animation_movement(
    mut q_moving_entities: Query<(&MovementIntent, &mut AseAnimation, &CanMove), Added<MoveToLocation>>,
) {
    for (movement_intent, mut ase_animation, can_move) in q_moving_entities.iter_mut() {
        if movement_intent.0 == Vec2::new(1., 0.) && !can_move.right.is_empty() {
            ase_animation.animation.play_loop(&can_move.right);
        }

        if movement_intent.0 == Vec2::new(-1., 0.) && !can_move.left.is_empty() {
            ase_animation.animation.play_loop(&can_move.left);
        }

        if movement_intent.0 == Vec2::new(0., 1.) && !can_move.up.is_empty() {
            ase_animation.animation.play_loop(&can_move.up);
        }

        if movement_intent.0 == Vec2::new(0., -1.) && !can_move.down.is_empty() {
            ase_animation.animation.play_loop(&can_move.down);
        }
    }
}

pub fn stop_animation_movement(
    mut removed: RemovedComponents<MoveToLocation>,
    mut q_animateable_entities: Query<&mut AseAnimation>
) {
    for entity in removed.read() {
        let Ok(mut ase_animation) = q_animateable_entities.get_mut(entity) else {
            return;
        };

        ase_animation.animation.clear_queue();
    }
}

pub fn start_animation_idle(
    mut q_animateable_entities: Query<&mut AseAnimation>
) {
    for mut ase_animation in q_animateable_entities {
        if !ase_animation.animation.playing {
            ase_animation.animation.play_loop("idle");
        }
    }
}

pub fn pause_animation(
    mut commands: Commands,
    mut q_animateable_entities: Query<(Entity, &mut AseAnimation)>
) {
    for (entity,  mut ase_animation) in q_animateable_entities {
        if ase_animation.animation.playing {
            commands.entity(entity).insert(PausedByState);
            ase_animation.animation.playing = false;
        }
    }
}

pub fn unpause_animation(
    mut commands: Commands,
    mut q_animateable_entities: Query<(Entity, &mut AseAnimation), With<PausedByState>>
) {
    for (entity, mut ase_animation) in q_animateable_entities {
        if ase_animation.animation.playing {
            commands.entity(entity).remove::<PausedByState>();
            ase_animation.animation.playing = true;
        }
    }
}
