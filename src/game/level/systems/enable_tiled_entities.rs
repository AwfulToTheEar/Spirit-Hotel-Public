use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use bevy_ecs_tiled::prelude::{TiledMapLayer, TiledMapMarker, TiledMapObject};

use crate::game::level::components::prelude::DisabledByVisibility;

pub fn enable_tiled_entities(
    mut commands: Commands,
    q_rendereable_entities: Query<
        Entity,
        (
            With<DisabledByVisibility>,
            With<Disabled>,
            Or<(
                With<TiledMapMarker>,
                With<TiledMapLayer>,
                With<TiledMapObject>,
            )>,
        ),
    >,
) {
    for entity in q_rendereable_entities {
        commands
            .entity(entity)
            .remove::<(Disabled, DisabledByVisibility)>();
    }
}
