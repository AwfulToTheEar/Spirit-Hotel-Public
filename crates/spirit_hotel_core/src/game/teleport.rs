use bevy::prelude::*;

pub fn teleport(
    mut commands: Commands,
    mut teleportables: Query<(Entity, &GlobalTransform, &mut Transform, &Teleport)>,
) {
    for (entity, global_transform, mut transform, teleport) in teleportables.iter_mut() {
        transform.translation =
            teleport.0 - (global_transform.translation() - transform.translation);

        commands.entity(entity).remove::<Teleport>();
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component, Default)]
pub struct Teleport(pub Vec3);
