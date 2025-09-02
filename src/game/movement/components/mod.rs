mod CanMove;
mod CanPush;
mod MoveToLocation;
mod MovementDuration;
mod MovementIntent;
mod Pushable;
pub mod prelude {
    pub use super::CanMove::*;
    pub use super::CanPush::*;
    pub use super::MoveToLocation::*;
    pub use super::MovementDuration::*;
    pub use super::MovementIntent::*;
    pub use super::Pushable::*;
}
