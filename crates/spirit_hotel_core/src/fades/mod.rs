mod fade_in;
mod fade_out;

use std::marker::PhantomData;
use bevy::camera::visibility::RenderLayers;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bitvec::prelude::BitVec;
use crate::{screens::Screen, theme::palette::FADE_BACKGROUND_COLOR};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Fade>();
    app.init_resource::<FadeData>();
    app.init_resource::<FadeTasks>();

    app.add_plugins((fade_in::plugin, fade_out::plugin))
        .add_systems(Startup, spawn_fade_camera);
}

#[derive(Resource, Reflect)]
#[reflect(Resource, Default)]
pub struct FadeData {
    pub color: Color,
    pub duration: f32
}

impl Default for FadeData {
    fn default() -> Self {
        Self {
            color: FADE_BACKGROUND_COLOR,
            duration: 0.3
        }
    }
}

#[derive(Resource)]
pub struct FadeToScreen(pub Screen);

#[derive(Resource)]
pub struct FadeTask<R: Resource> {
    marker: PhantomData<R>,
    pub key: FadeTaskKey,
}

impl<R: Resource> FadeTask<R> {
    pub fn new(key: FadeTaskKey) -> Self {
        Self { marker: PhantomData, key }
    }
}

#[derive(Copy, Clone, Reflect, Default, Debug)]
#[reflect(Default)]
pub struct FadeTaskKey(pub usize);
#[derive(Resource, Default)]
pub struct FadeTasks(BitVec);

impl FadeTasks {
    pub fn take(&mut self)-> FadeTaskKey {
        let index = self.0.len();
        self.0.set(index, false);
        FadeTaskKey(index)
    }

    pub fn complete(&mut self, key: &FadeTaskKey){
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
        Self { marker: PhantomData }
    }
}

pub fn complete_fade_task<R: Resource> (
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
            order: 1,
            ..default()
        },
        Camera2d,
        RenderLayers::layer(1)
    ));
}