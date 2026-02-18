use crate::components::prelude::TriggerHeldFor;
use bevy::prelude::*;
use std::ops::Add;

pub fn tick_trigger_held_for(time: Res<Time>, mut q_trigger_held_for: Query<&mut TriggerHeldFor>) {
    for mut trigger_held_for in q_trigger_held_for.iter_mut() {
        trigger_held_for.0 = trigger_held_for.0.add(time.delta());
    }
}
