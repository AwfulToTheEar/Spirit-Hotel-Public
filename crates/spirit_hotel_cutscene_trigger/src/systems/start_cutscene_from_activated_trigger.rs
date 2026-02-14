use bevy::prelude::*;
use spirit_hotel_core::fades::{FadeTask, FadeTasks};
use crate::components::prelude::StartCutscene;
use spirit_hotel_cutscene::resources::prelude::SpawnCutscene;
use spirit_hotel_trigger::components::prelude::{FromTrigger, IsTrigger};
use spirit_hotel_trigger::events::prelude::TriggerActivated;
use std::path::PathBuf;
use spirit_hotel_core::dismissibles::Dismissible;
use spirit_hotel_cutscene::components::prelude::Cutscene;

pub fn start_cutscene_from_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    mut fade_tasks: ResMut<FadeTasks>,
    q_cutscene_starters: Query<(&StartCutscene, &FromTrigger), With<IsTrigger>>,
) {
    let Ok((start_cutscene, from_trigger)) = q_cutscene_starters.get(e_trigger_activated.trigger) else {
        return;
    };

    let mut path = PathBuf::from("tiled");
    path.push("worlds");
    path.push(start_cutscene.0.clone());

    match from_trigger {
        FromTrigger::FadeIn => commands.insert_resource(FadeTask::<SpawnCutscene>::new(fade_tasks.take())),
        FromTrigger::OpenDismissibles => commands.insert_resource(Dismissible::<Cutscene>::new())
    }
    
    commands.insert_resource(SpawnCutscene(path));

    return;
}
