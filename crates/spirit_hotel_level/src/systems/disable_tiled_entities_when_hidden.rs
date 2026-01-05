use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use bevy_ecs_tiled::prelude::{TiledLayer, TiledMap, TiledObject};
use crate::components::prelude::{DisabledByVisibility, TiledCollidersSpawned};

pub fn disable_tiled_entities_when_hidden(
    mut commands: Commands,
    q_rendereable_entities: Query<
        (Entity, &Visibility, &InheritedVisibility),
        (
            With<TiledCollidersSpawned>,
            Or<(
                With<TiledMap>,
                With<TiledLayer>,
                With<TiledObject>,
            )>,
        ),
    >,
) {
    for (entity, visibility, inherited_visibility) in q_rendereable_entities {
        if visibility == Visibility::Visible
            || visibility == Visibility::Inherited && inherited_visibility.get()
        {
            continue;
        }

        commands
            .entity(entity)
            .insert((Disabled, DisabledByVisibility));
    }
}
