#![allow(non_snake_case)]

use tao::event_loop::{EventLoopBuilder, ControlFlow};
use tao::window::WindowBuilder;
use tao::event::Event;
use wgpu::{Instance, Backends};

pub fn run() {

    let instances = Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::all(),
        flags: Default::default(),
        dx12_shader_compiler: Default::default(),
        gles_minor_version: Default::default()
    });

    for adapter in instances.enumerate_adapters(Backends::all()){
        println!("{:?}", adapter.get_info())
    }

}

pub fn openWindow(){

    env_logger::init();

    let event_loop = EventLoopBuilder::new().build();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("GammaRayEngine");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        };
    });

}

fn main() {
    run();
    openWindow();
}
