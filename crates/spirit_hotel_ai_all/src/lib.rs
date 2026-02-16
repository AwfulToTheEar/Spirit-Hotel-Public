mod components;
mod systems;

use crate::systems::prelude::setup_goap_for_brain;
use bevy::prelude::*;
use bevy_dogoap::prelude::*;

pub struct SpiritHotelAiAllPlugin;

impl Plugin for SpiritHotelAiAllPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(setup_goap_for_brain);
    }
}
