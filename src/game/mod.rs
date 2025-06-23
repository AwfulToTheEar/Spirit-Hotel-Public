//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use std::env;

use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_ecs_tiled::map;

use crate::screens::Screen;

pub mod level;
mod trigger;
mod movement;
mod player;
mod animation;
mod dialogue;
mod path_finding;

pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))

        .add_plugins((
            level::plugin,
            trigger::plugin,
            movement::plugin,
            player::plugin,
            animation::plugin,
            dialogue::plugin,
            path_finding::plugin
        ))

        .configure_sets(PreUpdate,
            GameplaySystems
                .run_if(in_state(Screen::Gameplay))
        )
        .configure_sets(Update,
            GameplaySystems
                .run_if(in_state(Screen::Gameplay))
        )
        .configure_sets(FixedUpdate,
            GameplaySystems
                .run_if(in_state(Screen::Gameplay))
        )

        .add_systems(Startup,
            |reg: Res<AppTypeRegistry>| {
                let mut path = env::current_dir().unwrap();
                path.push("tiled_types_filtered_export.json");

                info!("Export Tiled types to '{:?}'", &path);
                map::export_types(&reg, path, |name| {
                    if name.contains("spirit_hotel::") {
                        return true;
                    }

                    if name.contains("bevy_render::view::visibility::Visibility") {
                        return true;
                    }

                    return false;
                });
            },
        );
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct GameplaySystems;
