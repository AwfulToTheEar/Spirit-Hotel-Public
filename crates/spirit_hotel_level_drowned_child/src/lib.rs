use crate::systems::prelude::*;
use bevy::prelude::*;
use spirit_hotel_core::game::GameplaySystems;
use spirit_hotel_level::system_sets::prelude::LevelSystems;

pub mod systems;

pub struct SpiritHotelLevelDrownedChildPlugin;

impl Plugin for SpiritHotelLevelDrownedChildPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_world_init_breath_timer)
            .add_systems(
                Update,
                (handle_world_init_kagume_kagume, start_kagume_kagume_cycle)
                    .in_set(LevelSystems)
                    .in_set(GameplaySystems),
            );
    }
}
