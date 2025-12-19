use std::marker::PhantomData;
use bevy::prelude::*;

use crate::game::movement::components::prelude::SyncMovement;
#[derive(Component, Reflect)]
#[reflect(Component, Default)]
#[require(SyncMovement)]
pub struct SyncMovementWith<C: Component> {
    marker: PhantomData<C>,
}

impl<C: Component> Default for SyncMovementWith<C> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}