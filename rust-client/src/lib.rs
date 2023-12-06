use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

pub mod enet;
pub mod entity;
pub mod network;
pub mod play_node;
pub mod root;
