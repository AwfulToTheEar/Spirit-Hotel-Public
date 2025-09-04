use crate::game::GameplaySystems;
use crate::game::drowned_child::components::prelude::{BreathTimer, Breathing, InitBreathTimer};
use crate::game::drowned_child::resources::prelude::*;
use crate::game::drowned_child::systems::prelude::{
    add_breath_input_actions, lose_game_breath_timer_not_held, lose_game_breath_timer_timeout,
    remove_breath_input_actions, spawn_breath_timer_ui, tick_breath_timer,
    update_breath_timer_indicator,
};
use crate::{AppSystems, PausableSystems};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub mod components;
pub mod input;
pub mod resources;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.register_type::<Mean>()
        .register_type::<BreathTimer>()
        .register_type::<InitBreathTimer>()
        .add_input_context::<Breathing>()
        .add_systems(
            Update,
            (
                (
                    tick_breath_timer.in_set(AppSystems::TickTimers),
                    (
                        update_breath_timer_indicator,
                        lose_game_breath_timer_timeout,
                        lose_game_breath_timer_not_held,
                    )
                        .in_set(AppSystems::Update),
                )
                    .in_set(PausableSystems),
                (add_breath_input_actions, remove_breath_input_actions).in_set(AppSystems::Update),
                spawn_breath_timer_ui,
            )
                .in_set(GameplaySystems),
        );
}
