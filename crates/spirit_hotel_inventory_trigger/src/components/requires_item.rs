use bevy::prelude::*;
use spirit_hotel_trigger::components::prelude::WaitForExternalAction;

#[derive(Component, Reflect)]
#[require(WaitForExternalAction)]
#[reflect(Component)]
pub struct RequiresItem(pub String);
