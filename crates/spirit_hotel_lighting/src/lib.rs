use bevy::prelude::*;
use bevy_light_2d::prelude::*;

pub struct SpiritHotelLightingPlugin;

impl Plugin for SpiritHotelLightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Light2dPlugin);
    }
}
