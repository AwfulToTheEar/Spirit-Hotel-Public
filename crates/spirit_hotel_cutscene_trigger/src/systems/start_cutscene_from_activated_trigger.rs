use std::path::PathBuf;
use bevy::prelude::*;
use spirit_hotel_core::fades::{Fade};
use spirit_hotel_cutscene::components::prelude::StartCutscene;
use spirit_hotel_cutscene::resources::prelude::FadeToCutscene;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::messages::prelude::TriggerActivatedMessage;

pub fn start_cutscene_from_activated_trigger(
    mut commands: Commands,
    mut next_fade: ResMut<NextState<Fade>>,
    mut e_trigger_activated: MessageReader<TriggerActivatedMessage>,
    q_cutscene_starters: Query<&StartCutscene, With<IsTrigger>>,
) {
    for e in e_trigger_activated.read() {
        info!("Trigger activated: {:?}", e.trigger);
        let Ok(start_cutscene) = q_cutscene_starters.get(e.trigger) else {
            continue;
        };

        let mut path = PathBuf::from("tiled");
        path.push("worlds");
        path.push(start_cutscene.0.clone());

        commands.insert_resource(FadeToCutscene(path));
        next_fade.set(Fade::In);

        return;
    }
}