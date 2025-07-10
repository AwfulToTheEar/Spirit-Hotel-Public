mod add_scene_from_dialogue_command;
mod remove_scene_from_dialogue_command;
mod toggle_scene_from_dialogue_command;

pub mod prelude {
    pub use super::add_scene_from_dialogue_command::*;
    pub use super::remove_scene_from_dialogue_command::*;
    pub use super::toggle_scene_from_dialogue_command::*;
}
