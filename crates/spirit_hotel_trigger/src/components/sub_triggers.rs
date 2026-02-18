use std::slice;
use bevy::prelude::*;
use crate::components::prelude::SubTriggerOf;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = SubTriggerOf)]
pub struct SubTriggers(Vec<Entity>);

impl<'a> IntoIterator for &'a SubTriggers {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, Entity>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}