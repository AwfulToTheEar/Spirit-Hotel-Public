use crate::game::GameplaySystems;
use crate::game::drowned_child::components::prelude::{BreathTimer, InitBreathTimer};
use crate::game::drowned_child::resources::prelude::*;
use crate::game::drowned_child::systems::prelude::{
    spawn_breath_timer_ui, tick_breath_timer, update_breath_timer_indicator,
};
use crate::{AppSystems, PausableSystems};
use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.register_type::<Mean>()
        .register_type::<BreathTimer>()
        .register_type::<InitBreathTimer>()
        .add_systems(
            Update,
            (
                (
                    tick_breath_timer.in_set(AppSystems::TickTimers),
                    update_breath_timer_indicator.in_set(AppSystems::Update),
                )
                    .in_set(PausableSystems),
                spawn_breath_timer_ui,
            )
                .in_set(GameplaySystems),
        );
}
