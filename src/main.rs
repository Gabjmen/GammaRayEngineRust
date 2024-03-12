pub mod camera;
pub mod hdr;
pub mod model;
pub mod open_editor;
pub mod renderer;
pub mod resources;
pub mod texture;

fn main() {
    //showGPU();
    //open_editor::main();
    pollster::block_on(renderer::run());
}