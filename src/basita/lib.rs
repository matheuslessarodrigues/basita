// external
pub extern crate sdl2;

#[macro_use]
pub extern crate serde_derive;

pub extern crate serde;
pub extern crate serde_json;

pub extern crate uuid;

// internal
pub mod systems;
pub mod components;
pub mod events;

pub mod resources;
pub mod input;
pub mod math;

pub mod sdl;

mod engine;
pub use self::engine::*;