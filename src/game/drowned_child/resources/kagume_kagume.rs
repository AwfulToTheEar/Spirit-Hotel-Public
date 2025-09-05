use bevy::prelude::*;

#[derive(Resource, Clone, Debug, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct KagumeKagume {
    pub is_oni: bool,
}
