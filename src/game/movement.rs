use avian2d::{collision::collider::contact_query, prelude::Collider};
use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    game::{
        GameplaySystems,
        path_finding::{GridTileSize, ScanCollider},
        trigger::TriggerCollider,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CanMove>()
        .register_type::<MovementDuration>()
        .add_systems(
            Update,
            apply_intent
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems),
        )
        .add_systems(
            FixedUpdate,
            (
                tick_move_to_location.in_set(AppSystems::TickTimers),
                (apply_move_to_location, remove_move_to_location)
                    .chain()
                    .in_set(AppSystems::Update),
            )
                .chain()
                .in_set(GameplaySystems)
                .in_set(PausableSystems),
        );
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(MovementDuration)]
pub struct CanMove {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
}

#[derive(Component)]
pub struct MoveToLocation {
    pub timer: Timer,
    pub start: Vec3,
    pub end: Vec3,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementIntent(pub Vec2);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementDuration(pub f32);

impl Default for MovementDuration {
    fn default() -> Self {
        Self(0.2)
    }
}

pub fn apply_intent(
    mut commands: Commands,
    s_grid_tile_size: Single<&GridTileSize>,
    q_entities_with_intent: Query<
        (
            Entity,
            &MovementIntent,
            &MovementDuration,
            &Transform,
            &Children,
        ),
        (With<CanMove>, Without<MoveToLocation>),
    >,
    q_colliders: Query<
        (Entity, &Collider, &GlobalTransform),
        (Without<TriggerCollider>, Without<ScanCollider>),
    >,
) {
    let grid_tile_size = s_grid_tile_size.into_inner();

    for (entity, intent, duration, transform, children) in q_entities_with_intent.iter() {
        let step = (intent.0 * grid_tile_size.0.as_vec2()).extend(0.);

        let mut collider_found = false;
        for child in children.iter() {
            let Ok((_, collider, collider_transform)) = q_colliders.get(child) else {
                continue;
            };

            for (other_entity, other_collider, other_transform) in q_colliders.iter() {
                if children.contains(&other_entity) {
                    continue;
                }

                let Ok(result) = contact_query::intersection_test(
                    collider,
                    (collider_transform.translation() + step).truncate(),
                    collider_transform.rotation(),
                    other_collider,
                    other_transform.translation().truncate(),
                    other_transform.rotation(),
                ) else {
                    continue;
                };

                if result {
                    collider_found = true;
                    commands.entity(entity).remove::<MovementIntent>();
                    break;
                }
            }
        }

        if collider_found {
            continue;
        }

        commands.entity(entity).insert(MoveToLocation {
            timer: Timer::from_seconds(duration.0, TimerMode::Once),
            start: transform.translation,
            end: transform.translation + step,
        });
    }
}

pub fn tick_move_to_location(time: Res<Time>, mut q_timers: Query<&mut MoveToLocation>) {
    for mut move_to_location in q_timers.iter_mut() {
        move_to_location.timer.tick(time.delta());
    }
}

pub fn apply_move_to_location(
    mut q_moving_transforms: Query<(&mut MoveToLocation, &mut Transform)>,
) {
    for (move_to_location, mut transform) in q_moving_transforms.iter_mut() {
        transform.translation = move_to_location
            .start
            .lerp(move_to_location.end, move_to_location.timer.fraction());
    }
}

pub fn remove_move_to_location(
    mut commands: Commands,
    q_timers: Query<(Entity, &MoveToLocation), With<MovementIntent>>,
) {
    for (entity, move_to_location) in q_timers.iter() {
        if !move_to_location.timer.finished() {
            continue;
        }

        commands
            .entity(entity)
            .remove::<(MovementIntent, MoveToLocation)>();
    }
}
