use bevy::prelude::*;
use spirit_hotel_core::fades::Fade;
use crate::components::prelude::StartCutscene;
use spirit_hotel_cutscene::resources::prelude::FadeToCutscene;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::events::prelude::TriggerActivated;
use std::path::PathBuf;

pub fn start_cutscene_from_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    q_cutscene_starters: Query<&StartCutscene, With<IsTrigger>>,
) {
    let Ok(start_cutscene) = q_cutscene_starters.get(e_trigger_activated.trigger) else {
        return;
    };

    let mut path = PathBuf::from("tiled");
    path.push("worlds");
    path.push(start_cutscene.0.clone());

    commands.insert_resource(FadeToCutscene(path));

    return;
}
