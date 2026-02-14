mod fade_in;
mod fade_out;
mod fade_waiting;

use crate::{AppSystems, screens::Screen, theme::palette::FADE_BACKGROUND_COLOR};
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::*;
use bitvec::prelude::BitVec;
use std::marker::PhantomData;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Fade>();
    app.init_resource::<FadeData>();
    app.init_resource::<FadeTasks>();

    app.add_plugins((fade_in::plugin, fade_waiting::plugin, fade_out::plugin))
        .add_observer(complete_fade_task::<Delay>)
        .add_fade_task_update::<Delay>((
            tick_delay.in_set(AppSystems::TickTimers),
            complete_delay.in_set(AppSystems::Update),
        ))
        .add_systems(Startup, spawn_fade_camera);
}

//TODO: find better name for this
pub trait AddFadeTask<M> {
    fn add_fade_task_enter<R: Resource>(
        &mut self,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;

    fn add_fade_task_update<R: Resource>(
        &mut self,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;
}

impl<M> AddFadeTask<M> for App {
    fn add_fade_task_enter<R: Resource>(
        &mut self,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.add_systems(
            OnEnter(Fade::Waiting),
            systems.run_if(resource_exists::<R>.and(resource_exists::<FadeTask<R>>)),
        );

        self
    }

    fn add_fade_task_update<R: Resource>(
        &mut self,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.add_systems(
            Update,
            systems.run_if(resource_exists::<R>.and(resource_exists::<FadeTask<R>>)),
        );

        self
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource, Default)]
pub struct FadeData {
    pub color: Color,
    pub duration: f32,
}

impl Default for FadeData {
    fn default() -> Self {
        Self {
            color: FADE_BACKGROUND_COLOR,
            duration: 0.3,
        }
    }
}

#[derive(Resource)]
pub struct ChangeScreen(pub Screen);

#[derive(Resource)]
pub struct Delay(pub Timer);

#[derive(Resource)]
pub struct FadeTask<R: Resource> {
    marker: PhantomData<R>,
    pub key: FadeTaskKey,
}

impl<R: Resource> FadeTask<R> {
    pub fn new(key: FadeTaskKey) -> Self {
        Self {
            marker: PhantomData,
            key,
        }
    }
}

#[derive(Copy, Clone, Reflect, Default, Debug)]
#[reflect(Default)]
pub struct FadeTaskKey(pub usize);
#[derive(Resource, Default)]
pub struct FadeTasks(BitVec);

impl FadeTasks {
    pub fn take(&mut self) -> FadeTaskKey {
        info!("Taking fade task");
        let index = self.0.len();
        self.0.insert(index, false);
        FadeTaskKey(index)
    }

    pub fn complete(&mut self, key: &FadeTaskKey) {
        info!("Completing fade task");
        self.0.set(key.0, true);
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Fade {
    #[default]
    None,
    In,
    Waiting,
    Out,
}

#[derive(Event)]
pub struct FadeTaskFinished<R: Resource> {
    marker: PhantomData<R>,
}

impl<R: Resource> FadeTaskFinished<R> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

pub fn tick_delay(time: Res<Time>, mut delay: ResMut<Delay>) {
    delay.0.tick(time.delta());
}

pub fn complete_delay(mut commands: Commands, delay: Res<Delay>) {
    if !delay.0.just_finished() {
        return;
    }

    commands.trigger(FadeTaskFinished::<Delay>::new());
}

pub fn complete_fade_task<R: Resource>(
    _: On<FadeTaskFinished<R>>,
    mut commands: Commands,
    mut fade_out_tasks: ResMut<FadeTasks>,
    fade_task: Res<FadeTask<R>>,
) {
    fade_out_tasks.complete(&fade_task.key);
    commands.remove_resource::<FadeTask<R>>();
    commands.remove_resource::<R>();
}

fn spawn_fade_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Fade Camera"),
        Camera {
            order: 2,
            ..default()
        },
        Camera2d,
        RenderLayers::layer(2),
    ));
}
