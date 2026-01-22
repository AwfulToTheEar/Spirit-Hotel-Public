use std::path::PathBuf;

use bevy::prelude::*;
use spirit_hotel_core::fades::{FadeToScreen, ReadyForFadeOut};
use spirit_hotel_core::screens::Screen;
use spirit_hotel_level::resources::prelude::LevelAssets;
use spirit_hotel_trigger::components::prelude::IsTrigger;
use spirit_hotel_trigger::events::prelude::TriggerActivated;
use crate::components::prelude::LoadLevel;

pub fn load_level_from_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_assets: ResMut<LevelAssets>,
    mut ready_for_fade_out: ResMut<ReadyForFadeOut>,
    q_map_loaders: Query<&LoadLevel, With<IsTrigger>>,
) {
    let Ok(load_level) = q_map_loaders.get(e_trigger_activated.trigger) else {
        return;
    };

    let mut path = PathBuf::from("tiled");
    path.push("worlds");
    path.push(load_level.0.clone());

    level_assets.world = Some(asset_server.load(path));

    commands.insert_resource(FadeToScreen(Screen::Loading));
    ready_for_fade_out.0.insert("screen".to_string(), false);

    return;
}
