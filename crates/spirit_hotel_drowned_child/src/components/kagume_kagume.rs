use bevy::prelude::*;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct KagumeKagume {
    pub is_oni: bool,
    pub dance_cycles: Vec<f32>
}

impl Default for KagumeKagume {
    fn default() -> Self {
        Self { is_oni: false, dance_cycles: Vec::<f32>::from([10.0, 15.0, 8.0]) }
    }
}