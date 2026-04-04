mod activate_on_external_action_completed;
mod activate_when_sub_triggers_active;
mod add_delay;
mod continuous_trigger;
mod from_trigger;
mod inside_triggers;
mod interact_trigger;
mod is_trigger;
mod spawn_image;
mod sub_trigger_of;
mod sub_triggers;
mod throttle_trigger;
mod throttle_trigger_timer;
mod trigger_collider;
mod trigger_for;
mod trigger_has_been_activated;
mod trigger_held_for;
mod unlock_on_external_action_completed;
mod wait_for_external_action;

pub mod prelude {
    pub use super::activate_when_sub_triggers_active::*;
    pub use super::add_delay::*;
    pub use super::continuous_trigger::*;
    pub use super::from_trigger::*;
    pub use super::inside_triggers::*;
    pub use super::interact_trigger::*;
    pub use super::is_trigger::*;
    pub use super::spawn_image::*;
    pub use super::sub_trigger_of::*;
    pub use super::sub_triggers::*;
    pub use super::throttle_trigger::*;
    pub use super::throttle_trigger_timer::*;
    pub use super::trigger_collider::*;
    pub use super::trigger_for::*;
    pub use super::trigger_has_been_activated::*;
    pub use super::trigger_held_for::*;
    pub use super::wait_for_external_action::*;
}
