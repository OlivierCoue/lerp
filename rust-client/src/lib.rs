use godot::prelude::*;

pub mod enet;
pub mod entity;
pub mod network;
pub mod play_node;
pub mod root;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
