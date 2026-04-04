use bevy::prelude::*;
use bevy_av1::VideoPlayer;

pub fn unpause_cutscene(mut q_video_players: Query<&mut VideoPlayer>) {
    for mut player in q_video_players.iter_mut() {
        player.paused = false;
    }
}
