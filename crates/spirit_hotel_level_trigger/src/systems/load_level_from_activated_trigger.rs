use std::path::PathBuf;

use bevy::prelude::*;
use spirit_hotel_core::fades::{Fade, FadeData};
use spirit_hotel_core::screens::Screen;
use spirit_hotel_level::components::prelude::LoadLevel;
use spirit_hotel_level::resources::prelude::LevelAssets;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::messages::prelude::TriggerActivatedMessage;

pub fn load_level_from_activated_trigger(
    asset_server: Res<AssetServer>,
    mut level_assets: ResMut<LevelAssets>,
    mut fade_data: ResMut<FadeData>,
    mut next_fade: ResMut<NextState<Fade>>,
    mut e_trigger_activated: MessageReader<TriggerActivatedMessage>,
    q_map_loaders: Query<&LoadLevel, With<IsTrigger>>,
) {
    for e in e_trigger_activated.read() {
        let Ok(load_level) = q_map_loaders.get(e.trigger) else {
            continue;
        };

        let mut path = PathBuf::from("tiled");
        path.push("worlds");
        path.push(load_level.0.clone());

        level_assets.world = Some(asset_server.load(path));

        fade_data.next_state = Some(Screen::Loading);
        next_fade.set(Fade::In);

        return;
    }
}
