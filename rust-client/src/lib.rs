use godot::prelude::*;

pub mod enet;
pub mod entity;
pub mod network;
pub mod play_node;
pub mod root;
pub mod server_entity;
pub mod utils;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
