use crate::systems::prelude::*;
use bevy::prelude::*;
use bevy_av1::VideoPlugin;
use spirit_hotel_core::fades::Fade;
use spirit_hotel_core::screens::Screen;
use crate::components::prelude::StartCutscene;
use crate::resources::prelude::FadeToCutscene;

pub mod resources;

pub mod systems;

pub mod components;

pub struct SpiritHotelCutscenePlugin;

impl Plugin for SpiritHotelCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VideoPlugin)
            .add_observer(fade_out_on_cutscene_end)
            .add_systems(OnTransition {exited: Fade::In, entered: Fade::None}, spawn_cutscene.run_if(resource_exists::<FadeToCutscene>));
    }
}