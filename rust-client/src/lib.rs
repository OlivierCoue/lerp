use godot::prelude::*;

pub mod auth;
pub mod lobby;
pub mod network;
pub mod play;
pub mod root;
pub mod utils;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
