use bevy::{platform::collections::HashMap, prelude::*};
use bevy_aseprite_ultra::prelude::Aseprite;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct AnimationAssets(pub HashMap<Entity, Handle<Aseprite>>);

impl FromWorld for AnimationAssets {
    fn from_world(_world: &mut World) -> Self {
        Self(HashMap::<Entity, Handle<Aseprite>>::default())
    }
}
