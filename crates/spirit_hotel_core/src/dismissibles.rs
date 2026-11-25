use crate::fades::FadeTask;
use crate::game::GameplaySystems;
use crate::theme::palette::DISMISSIBLE_BACKGROUND_COLOR;
use crate::theme::widget;
use crate::{AppSystems, OverlayCamera, OverlayRoot};
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::ScheduleSystem;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<DismissiblesOpen>()
        .add_systems(OnEnter(DismissiblesOpen(true)), spawn_dismissible_root)
        .add_systems(
            Update,
            close_dismissibles
                .in_set(AppSystems::Update)
                .in_set(GameplaySystems)
                .run_if(input_just_pressed(KeyCode::Escape)),
        );
}

pub trait AddDismissible<M> {
    fn add_dismissible<R: Resource>(
        &mut self,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;
}

impl<M> AddDismissible<M> for App {
    fn add_dismissible<R: Resource>(
        &mut self,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.add_systems(
            OnEnter(DismissiblesOpen(true)),
            systems.run_if(resource_exists::<R>.and(not(resource_exists::<FadeTask<R>>))),
        )
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub struct DismissiblesOpen(pub bool);

pub fn spawn_dismissible_root(mut commands: Commands, s_overlay_camera: Single<Entity, With<OverlayCamera>>) {
    commands.spawn((
        widget::ui_root("Dismissible Root"),
        DespawnOnExit(DismissiblesOpen(true)),
        BackgroundColor(DISMISSIBLE_BACKGROUND_COLOR),
        UiTargetCamera(s_overlay_camera.entity()),
        OverlayRoot,
    ));
}

pub fn close_dismissibles(mut next_dismissibles: ResMut<NextState<DismissiblesOpen>>) {
    next_dismissibles.set(DismissiblesOpen(false));
}
