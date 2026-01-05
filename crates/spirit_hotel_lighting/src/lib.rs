use bevy::prelude::*;
use bevy_light_2d::prelude::*;

pub mod light_2d {
    pub use bevy_light_2d::*;
}

pub struct SpiritHotelLightingPlugin;

impl Plugin for SpiritHotelLightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Light2dPlugin);
    }
}
