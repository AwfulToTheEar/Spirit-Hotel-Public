pub mod components;

use crate::components::prelude::HasTarget;
use bevy::prelude::*;
use bevy_dogoap::prelude::*;

pub struct SpiritHotelAiPlugin;

impl Plugin for SpiritHotelAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DogoapPlugin::default());

        register_components!(app, [HasTarget]);
    }
}

#[derive(Clone, Default, Reflect, Copy, EnumDatum)]
pub enum Possible {
    #[default]
    Unknown,
    Yes,
    No,
}
