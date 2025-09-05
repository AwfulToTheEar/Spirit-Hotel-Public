use crate::game::GameplaySystems;
use crate::game::drowned_child::components::prelude::{
    BreathTimer, Breathing, InitBreathTimer, KagumeKagumeDancer,
};
use crate::game::drowned_child::resources::prelude::*;
use crate::game::drowned_child::systems::prelude::{
    add_block_movement_kagume_kagume_dancers, add_breath_input_actions,
    add_force_movement_kagume_kagume_dancers, add_kagume_kagume_input_actions,
    lose_game_breath_timer_not_held, lose_game_breath_timer_timeout, remove_breath_input_actions,
    remove_force_movement_kagume_kagume_dancers, remove_kagume_kagume_input_actions,
    spawn_breath_timer_ui, tick_breath_timer, toggle_block_movement_player_oni,
    toggle_halt_pathfinding_intent_kagume_kagume_dancers, update_breath_timer_indicator,
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
        .register_type::<KagumeKagume>()
        .register_type::<KagumeKagumeDancer>()
        .add_input_context::<Breathing>()
        .add_input_context::<KagumeKagumeDancer>()
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
                (
                    add_breath_input_actions,
                    remove_breath_input_actions,
                    add_block_movement_kagume_kagume_dancers
                        .run_if(not(resource_exists::<KagumeKagume>)),
                    (
                        add_force_movement_kagume_kagume_dancers,
                        add_kagume_kagume_input_actions,
                    )
                        .run_if(resource_added::<KagumeKagume>),
                    (
                        remove_force_movement_kagume_kagume_dancers,
                        remove_kagume_kagume_input_actions,
                    )
                        .run_if(resource_removed::<KagumeKagume>),
                    (
                        toggle_block_movement_player_oni,
                        toggle_halt_pathfinding_intent_kagume_kagume_dancers,
                    )
                        .run_if(resource_exists::<KagumeKagume>),
                )
                    .in_set(AppSystems::Update),
                spawn_breath_timer_ui,
            )
                .in_set(GameplaySystems),
        );
}
