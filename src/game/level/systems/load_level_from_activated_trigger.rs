use std::path::PathBuf;

use bevy::prelude::*;

use crate::{
    game::{
        level::{components::prelude::LoadLevel, resources::prelude::LevelAssets},
        trigger::{components::prelude::IsTrigger, events::prelude::TriggerActivatedEvent},
    },
    screens::Screen,
};

pub fn load_level_from_activated_trigger(
    asset_server: Res<AssetServer>,
    mut level_assets: ResMut<LevelAssets>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut e_trigger_activated: EventReader<TriggerActivatedEvent>,
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

        next_screen.set(Screen::Loading);

        return;
    }
}
