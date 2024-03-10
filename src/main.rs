#![allow(non_snake_case)]

use GammaRayEngineRust::open_editor;
use GammaRayEngineRust::renderer::run;

fn main() {
    //showGPU();
    open_editor::main();
    //pollster::block_on(run());
}
