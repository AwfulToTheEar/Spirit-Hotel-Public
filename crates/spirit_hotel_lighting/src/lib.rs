mod systems;

use crate::systems::prelude::add_ambient_light_to_main_camera;
use bevy::prelude::*;
use bevy_light_2d::prelude::*;

pub struct SpiritHotelLightingPlugin;

impl Plugin for SpiritHotelLightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Light2dPlugin)
            .add_observer(add_ambient_light_to_main_camera);
    }
}
