use bevy::prelude::*;
use spirit_hotel_core::fades::{Fade, FadeTask, FadeTasks};
use crate::components::prelude::StartCutscene;
use spirit_hotel_cutscene::resources::prelude::SpawnCutscene;
use spirit_hotel_trigger::components::prelude::{FadeInFromTrigger, IsTrigger};
use spirit_hotel_trigger::events::prelude::TriggerActivated;
use std::path::PathBuf;

pub fn start_cutscene_from_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    mut fade_tasks: ResMut<FadeTasks>,
    q_cutscene_starters: Query<(&StartCutscene, Has<FadeInFromTrigger>), With<IsTrigger>>,
) {
    let Ok((start_cutscene, has_fade_in_from_trigger)) = q_cutscene_starters.get(e_trigger_activated.trigger) else {
        return;
    };

    let mut path = PathBuf::from("tiled");
    path.push("worlds");
    path.push(start_cutscene.0.clone());

    if has_fade_in_from_trigger {
        commands.insert_resource(FadeTask::<SpawnCutscene>::new(fade_tasks.take()));
    }
    
    commands.insert_resource(SpawnCutscene(path));

    return;
}
