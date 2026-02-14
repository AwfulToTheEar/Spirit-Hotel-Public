use std::path::PathBuf;

use bevy::prelude::*;
use spirit_hotel_core::fades::{ChangeScreen, FadeTasks, FadeTask};
use spirit_hotel_core::screens::Screen;
use spirit_hotel_level::resources::prelude::LevelAssets;
use spirit_hotel_trigger::components::prelude::{FromTrigger, IsTrigger};
use spirit_hotel_trigger::events::prelude::TriggerActivated;
use crate::components::prelude::LoadLevel;

pub fn load_level_from_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_assets: ResMut<LevelAssets>,
    mut fade_out_tasks: ResMut<FadeTasks>,
    q_map_loaders: Query<(&LoadLevel, &FromTrigger), With<IsTrigger>>,
) {
    let Ok((load_level, from_trigger)) = q_map_loaders.get(e_trigger_activated.trigger) else {
        return;
    };

    let mut path = PathBuf::from("tiled");
    path.push("worlds");
    path.push(load_level.0.clone());

    level_assets.world = Some(asset_server.load(path));

    if matches!(from_trigger, FromTrigger::FadeIn) {
        commands.insert_resource(FadeTask::<ChangeScreen>::new(fade_out_tasks.take()));
    }

    commands.insert_resource(ChangeScreen(Screen::Loading));

    return;
}
