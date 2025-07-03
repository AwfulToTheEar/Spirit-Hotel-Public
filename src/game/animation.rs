use std::path::PathBuf;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

use crate::{asset_tracking::LoadResource, game::{level::LevelSystems, movement::{CanMove, MoveToLocation, MovementIntent}, GameplaySystems}, AppSystems, PausableSystems, Pause};

pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins((
            AsepriteUltraPlugin,
            FilterQueryInspectorPlugin::<With<Animatable>>::default()
        ))

        .register_type::<AnimationPath>()

        .load_resource::<AnimationAssets>()

        .add_systems(PreUpdate,
            init_animatable_entities
                .after(LevelSystems)
                .in_set(GameplaySystems)
        )

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

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct AnimationAssets(pub HashMap<Entity, Handle<Aseprite>>);

impl FromWorld for AnimationAssets {
    fn from_world(_world: &mut World) -> Self {
        Self(HashMap::<Entity, Handle<Aseprite>>::default())
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct PausedByState;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct Animatable;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(Animatable)]
pub struct AnimationPath(pub PathBuf);

pub fn init_animatable_entities(
    mut commands: Commands,
    animation_assets: Res<AnimationAssets>,
    q_animatables: Query<Entity, (With<Animatable>, Without<AseAnimation>)>,
) {
    for entity in q_animatables.iter() {
        let Some(animation_handle) = animation_assets.0.get(&entity) else {
            continue;
        };

        commands.entity(entity).insert(AseAnimation {
            aseprite: animation_handle.clone(),
            ..default()
        });
    }
}

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
            continue;
        };

        ase_animation.animation.stop();
    }
}

pub fn start_animation_idle(
    q_animateable_entities: Query<&mut AseAnimation>
) {
    for mut ase_animation in q_animateable_entities {
        if ase_animation.animation.playing {
            continue;
        }

        ase_animation.animation.play_loop("idle");
    }
}

pub fn pause_animation(
    mut commands: Commands,
    q_animateable_entities: Query<(Entity, &mut AseAnimation)>
) {
    for (entity,  mut ase_animation) in q_animateable_entities {
        if ase_animation.animation.playing {
            commands.entity(entity).insert(PausedByState);
            ase_animation.animation.pause();
        }
    }
}

pub fn unpause_animation(
    mut commands: Commands,
    q_animateable_entities: Query<(Entity, &mut AseAnimation), With<PausedByState>>
) {
    for (entity, mut ase_animation) in q_animateable_entities {
        if ase_animation.animation.playing {
            commands.entity(entity).remove::<PausedByState>();
            ase_animation.animation.start();
        }
    }
}
