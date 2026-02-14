use crate::AppSystems;
use crate::game::GameplaySystems;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use std::marker::PhantomData;
use bevy::ecs::system::ScheduleSystem;
use crate::fades::FadeTask;

pub fn plugin(app: &mut App) {
    app.init_state::<DismissiblesOpen>().add_systems(
        Update,
        close_dismissibles
            .in_set(AppSystems::Update)
            .in_set(GameplaySystems)
            .run_if(input_just_pressed(KeyCode::Escape)),
    );
}

pub trait AddDismissible<M> {
    fn add_dismissible<R: Resource, C: Component>(&mut self, systems: impl IntoScheduleConfigs<ScheduleSystem, M> ) -> &mut Self;
}

impl<M> AddDismissible<M> for App {
    fn add_dismissible<R: Resource, C: Component>(&mut self, systems: impl IntoScheduleConfigs<ScheduleSystem, M> ) -> &mut Self {
        self.add_systems(
            Update,
            mark_dismissible_despawn::<C>
                .in_set(GameplaySystems)
                .in_set(AppSystems::Update)
                .run_if(resource_exists::<Dismissible<C>>.and(in_state(DismissiblesOpen(true)))),
        ).add_systems(
            OnEnter(DismissiblesOpen(true)),
            systems.run_if(
                resource_exists::<R>
                    .and(resource_exists::<Dismissible<C>>)
                    .and(not(resource_exists::<FadeTask<R>>)),
            ),
        )
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub struct DismissiblesOpen(pub bool);

#[derive(Resource)]
pub struct Dismissible<C: Component>(PhantomData<C>);

impl<C: Component> Dismissible<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

pub fn mark_dismissible_despawn<C: Component>(
    mut commands: Commands,
    query: Query<Entity, (Added<C>, Without<DespawnOnExit<DismissiblesOpen>>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(DespawnOnExit(DismissiblesOpen(true)));
    }
}

pub fn close_dismissibles(mut next_dismissibles: ResMut<NextState<DismissiblesOpen>>) {
    next_dismissibles.set(DismissiblesOpen(false));
}
