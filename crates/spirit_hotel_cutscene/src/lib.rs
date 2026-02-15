use crate::resources::prelude::SpawnCutscene;
use crate::systems::prelude::*;
use bevy::prelude::*;
use bevy_av1::VideoPlugin;
use spirit_hotel_core::Pause;
use spirit_hotel_core::fades::RegisterFadeTask;

pub mod components;
pub mod resources;
pub mod systems;

pub struct SpiritHotelCutscenePlugin;

impl Plugin for SpiritHotelCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VideoPlugin)
            .add_observer(fade_out_on_cutscene_end)
            .register_fade_task::<SpawnCutscene>()
            .add_systems(OnEnter(Pause(true)), pause_cutscene)
            .add_systems(OnEnter(Pause(false)), unpause_cutscene)
            .add_observer(spawn_cutscene);
    }
}
