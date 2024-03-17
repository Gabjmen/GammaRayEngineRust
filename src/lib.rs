#![feature(async_closure)]
pub mod camera;
pub mod designer;
pub mod hdr;
pub mod model;
pub mod renderer;
pub mod resources;
pub mod texture;
pub mod entity;
pub mod gui;
#[path = "./test/test.rs"]
pub mod test;
#[path = "./test/framework.rs"]
pub mod framework;
#[path = "utils/fps_counter.rs"]
pub mod fps_counter;
#[path = "utils/take_fps.rs"]
pub mod take_fps;
