pub mod camera;
pub mod hdr;
pub mod model;
pub mod designer;
pub mod renderer;
pub mod resources;
pub mod texture;
pub mod gui;

fn main() {
    //showGPU();
    //open_editor::main();
    pollster::block_on(renderer::run());
}