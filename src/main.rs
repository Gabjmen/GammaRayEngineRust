#![allow(non_snake_case)]

use GammaRayEngineRust::{showGPU, run};

fn main() {
    showGPU();
    pollster::block_on(run());
}
