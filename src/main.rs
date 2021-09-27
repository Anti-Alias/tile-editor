use winit::window::{WindowBuilder, WindowId};
use winit::event::*;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use std::any::Any;
use tile_editor::{App, Texture, AppListener, AppResources, Renderer, RenderParams};
use pollster::block_on;
use log::info;
use winit::dpi::PhysicalSize;
use wgpu::{CommandEncoderDescriptor, RenderPass, PipelineLayoutDescriptor};

// Main application listener
struct MyListener {
    renderer: Option<Renderer>
}

impl AppListener for MyListener {

    fn on_start(&self, resources: &AppResources) {
        println!("App started");
    }

    fn on_draw(&self, render_pass: &mut RenderPass) {

        //self.renderer().render(render_pass, &params);
    }

    fn on_resize(&self, size: PhysicalSize<u32>, resources: &AppResources) {
    }
}

fn main() {

    // Sets up logging
    env_logger::init();

    // Creates app and starts it
    let listener = MyListener {
        renderer: None
    };
    let mut app = block_on(App::new(listener));
    app.start();
}