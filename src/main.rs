#![allow(non_snake_case)]

mod gammaray_open_app;

use std::env;
use GammaRayEngineRust::run;

fn main() {
    //showGPU();
    //gammaray_open_app::open_project_window();
    pollster::block_on(run());
}
