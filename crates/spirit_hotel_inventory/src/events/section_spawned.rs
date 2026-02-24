use crate::components::prelude::InventoryMenuSection;
use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct SectionSpawned {
    #[event_target]
    pub entity: Entity,
    pub section: InventoryMenuSection,
}
impl SectionSpawned {
    pub fn with_section(section: InventoryMenuSection) -> impl FnOnce(Entity) -> Self {
        move |entity| {
            info!("Section Spawned: {:?}", entity);
            Self { entity, section }
        }
    }
}
