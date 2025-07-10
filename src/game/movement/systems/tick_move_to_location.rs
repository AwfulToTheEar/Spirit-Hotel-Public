use bevy::prelude::*;

use crate::game::movement::components::prelude::MoveToLocation;

pub fn tick_move_to_location(time: Res<Time>, mut q_timers: Query<&mut MoveToLocation>) {
    for mut move_to_location in q_timers.iter_mut() {
        move_to_location.timer.tick(time.delta());
    }
}
