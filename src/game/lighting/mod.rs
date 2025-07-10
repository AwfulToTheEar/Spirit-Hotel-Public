use bevy::prelude::*;
use bevy_light_2d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(Light2dPlugin);
}
