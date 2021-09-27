use winit::window::{WindowBuilder, WindowId};
use winit::event::*;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use std::any::Any;
use tile_editor::{App, Texture, AppListener, AppResources, Renderer, DrawResources, RenderParams};
use pollster::block_on;
use log::info;
use winit::dpi::PhysicalSize;
use wgpu::CommandEncoderDescriptor;

// Main application listener
struct MyListener {
    //renderer: Renderer
}

impl AppListener for MyListener {

    fn on_start<'a>(&'a self, resources: &'a AppResources<'a>) {
        println!("App started");
    }

    fn on_draw<'a>(&'a self, draw_resources: &'a DrawResources<'a>) {

        let params = todo!();
        //self.renderer.render(draw_resources.render_pass, &params);
    }

    fn on_resize<'a>(&self, size: PhysicalSize<u32>, resources: &'a AppResources<'a>) {
    }
}

fn main() {

    // Sets up logging
    env_logger::init();

    // Creates app and starts it
    let listener = MyListener {

    };
    let mut app = block_on(App::new(listener));
    app.start();
}