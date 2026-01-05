mod activate_instant_triggers;
mod activate_interact_triggers;
mod debug_trigger_events;

pub mod prelude {
    pub use super::activate_instant_triggers::*;
    pub use super::activate_interact_triggers::*;
    pub use super::debug_trigger_events::*;
}
