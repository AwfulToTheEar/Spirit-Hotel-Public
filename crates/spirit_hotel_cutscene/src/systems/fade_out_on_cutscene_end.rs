use crate::resources::prelude::SpawnCutscene;
use bevy::prelude::*;
use bevy_av1::VideoPlayer;
use spirit_hotel_core::fades::{FadeTask, FadeTaskFinished};

pub fn fade_out_on_cutscene_end(
    _: On<Despawn, VideoPlayer>,
    mut commands: Commands,
    option_fade_task: Option<Res<FadeTask<SpawnCutscene>>>,
) {
    if option_fade_task.is_none() {
        return;
    }

    commands.trigger(FadeTaskFinished::<SpawnCutscene>::new());
}
