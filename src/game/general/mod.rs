use crate::game::general::resources::prelude::Insanity;
use bevy::prelude::*;

mod resources;

pub fn plugin(app: &mut App) {
    app.register_type::<Insanity>();
}
