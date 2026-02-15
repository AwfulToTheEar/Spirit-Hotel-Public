use crate::components::prelude::{FromTrigger, IsTrigger, SpawnImage};
use crate::events::prelude::TriggerActivated;
use bevy::prelude::*;
use spirit_hotel_core::fades::{FadeTask, FadeTasks};
use spirit_hotel_core::game::ShowImage;
use std::path::PathBuf;

pub fn spawn_image_on_activated_trigger(
    e_trigger_activated: On<TriggerActivated>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut fade_tasks: ResMut<FadeTasks>,
    q_image_showers: Query<(&SpawnImage, &FromTrigger), With<IsTrigger>>,
) {
    let Ok((spawn_image, from_trigger)) = q_image_showers.get(e_trigger_activated.trigger) else {
        return;
    };

    if matches!(from_trigger, FromTrigger::FadeIn) {
        commands.insert_resource(FadeTask::<ShowImage>::new(fade_tasks.take()))
    }

    let mut path = PathBuf::from("tiled");
    path.push("worlds");
    path.push(spawn_image.0.clone());

    commands.insert_resource(ShowImage(asset_server.load(path)));
}
