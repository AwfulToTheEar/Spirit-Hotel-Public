mod activate_triggers_on_enter;
mod activate_triggers_being_overlapped;
mod action_on_activated_trigger;
mod add_delay_on_activated_trigger;
mod spawn_image_on_activated_trigger;

pub mod prelude {
    pub use super::activate_triggers_on_enter::*;
    pub use super::activate_triggers_being_overlapped::*;
    pub use super::action_on_activated_trigger::*;
    pub use super::add_delay_on_activated_trigger::*;
    pub use super::spawn_image_on_activated_trigger::*;
}
