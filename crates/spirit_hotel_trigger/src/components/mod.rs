mod inside_triggers;
mod is_trigger;
mod trigger_collider;
mod from_trigger;
mod trigger_for;
mod interact_trigger;
mod continuous_trigger;
mod sub_trigger_of;
mod sub_triggers;
mod trigger_has_been_activated;
mod throttle_trigger;
mod add_delay;
mod spawn_image;

pub mod prelude {
    pub use super::inside_triggers::*;
    pub use super::is_trigger::*;
    pub use super::trigger_collider::*;
    pub use super::from_trigger::*;
    pub use super::trigger_for::*;
    pub use super::interact_trigger::*;
    pub use super::continuous_trigger::*;
    pub use super::sub_trigger_of::*;
    pub use super::sub_triggers::*;
    pub use super::trigger_has_been_activated::*;
    pub use super::throttle_trigger::*;
    pub use super::add_delay::*;
    pub use super::spawn_image::*;
}
