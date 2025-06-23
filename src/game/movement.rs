use avian2d::{collision::collider::contact_query, prelude::{Collider, ColliderDisabled}};
use bevy::prelude::*;
use bevy_ecs_tiled::{map::{self, TiledMapHandle}, prelude::TiledMap};

use crate::{game::{level::{InsideMap, LevelAssets}, path_finding::ScanCollider, trigger::TriggerCollider, GameplaySystems}, AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app
        .register_type::<CanMove>()
        .register_type::<MovementDuration>()

        .add_systems(Update, (
            (
                find_step_size.run_if(resource_exists::<LevelAssets>),
                apply_intent
            )
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .in_set(PausableSystems)
        ))

        .add_systems(FixedUpdate, (
            tick_move_to_location,
            apply_move_to_location,
            remove_move_to_location
        )
            .chain()
            .in_set(GameplaySystems)
            .in_set(PausableSystems)
        );
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
#[require(MovementDuration)]
pub struct CanMove {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String
}

#[derive(Component)]
pub struct MoveToLocation {
    pub timer: Timer,
    pub start: Vec3,
    pub end: Vec3
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementStepSize(Vec2);

pub fn apply_intent(
    mut commands: Commands,
    q_entities_with_intent: Query<(Entity, &MovementIntent, &MovementDuration, &MovementStepSize, &Transform, &Children), (With<CanMove>, Without<MoveToLocation>)>,
    q_colliders: Query<(Entity, &Collider, &GlobalTransform), (Without<TriggerCollider>, Without<ColliderDisabled>, Without<ScanCollider>)>,
) {
    for (entity, intent, duration,  step_size, transform, children) in q_entities_with_intent.iter() {
        let step = (intent.0 * step_size.0).extend(0.);

        for child in children.iter() {
            let Ok((_, collider, collider_transform)) = q_colliders.get(child) else {
                continue;
            };

            for (other_entity, other_collider, other_transform) in q_colliders.iter() {
                if children.contains(&other_entity) {
                    continue
                }

                let Ok(result) = contact_query::intersection_test(
                    collider,
                    (collider_transform.translation() + step).truncate(),
                    collider_transform.rotation(),
                    other_collider,
                    other_transform.translation().truncate(),
                    other_transform.rotation()
                ) else {
                    continue;
                };

                if result {
                    commands.entity(entity).remove::<MovementIntent>();
                    return;
                }
            }
        }

        commands.entity(entity).insert(MoveToLocation {
            timer: Timer::from_seconds(duration.0, TimerMode::Once),
            start: transform.translation,
            end: transform.translation + step
        });
    }
}

pub fn find_step_size(
    mut commands: Commands,
    tiled_maps: Res<Assets<TiledMap>>,
    mut q_movables: Query<(Entity, &InsideMap, Option<&mut MovementStepSize>), (With<CanMove>, Changed<InsideMap>)>,
    q_maps: Query<&TiledMapHandle>
) {
    for (entity, inside_map, step_size_option) in q_movables.iter_mut() {
        let Ok(map_handle) = q_maps.get(inside_map.0) else {
            continue;
        };

        let Some(tiled_map) = tiled_maps.get(map_handle.0.id()) else {
            continue;
        };

        let tile_size = map::utils::get_tile_size(&tiled_map.map);

        let Some(mut step_size) = step_size_option else {
            commands.entity(entity).insert(MovementStepSize(Vec2::new(tile_size.x, tile_size.y)));

            continue;
        };

        step_size.0 = Vec2::new(tile_size.x, tile_size.y);
    }
}

pub fn tick_move_to_location(
    time: Res<Time>,
    mut q_timers: Query<&mut MoveToLocation>
) {
    for mut move_to_location in q_timers.iter_mut() {
        move_to_location.timer.tick(time.delta());
    }
}

pub fn apply_move_to_location(
    mut q_moving_transforms: Query<(&mut MoveToLocation, &mut Transform)>,
) {
    for (move_to_location, mut transform) in q_moving_transforms.iter_mut() {
        transform.translation = move_to_location.start.lerp(move_to_location.end, move_to_location.timer.fraction());
    }
}

pub fn remove_move_to_location(
    mut commands: Commands,
    q_timers: Query<(Entity, &MoveToLocation), (With<MovementIntent>, With<MovementStepSize>)>
) {
    for (entity, move_to_location) in q_timers.iter() {
        if !move_to_location.timer.finished() {
            return
        }

        commands.entity(entity).remove::<(
            MovementIntent,
            MoveToLocation
        )>();
    }
}
