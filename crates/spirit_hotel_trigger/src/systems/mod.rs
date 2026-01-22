mod activate_instant_triggers;
mod activate_interact_triggers;
mod fade_in_on_activated_trigger;

pub mod prelude {
    pub use super::activate_instant_triggers::*;
    pub use super::activate_interact_triggers::*;
    pub use super::fade_in_on_activated_trigger::*;
}
