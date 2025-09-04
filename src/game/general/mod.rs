use crate::game::general::resources::prelude::Insanity;
use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;

mod resources;

pub fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .register_type::<Insanity>();
}
