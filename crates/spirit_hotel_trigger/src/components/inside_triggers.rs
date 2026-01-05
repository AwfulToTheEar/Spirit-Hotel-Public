use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct InsideTriggers {
    pub entities: Vec<Entity>,
}
