use bevy::prelude::*;
use bevy_light_2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Light2dPlugin);
}
