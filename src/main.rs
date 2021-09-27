use winit::window::{WindowBuilder, WindowId};
use winit::event::*;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use std::any::Any;
use tile_editor::{App, Texture, AppListener, AppResources, Renderer, RenderParams, RenderConfig};
use pollster::block_on;
use log::info;
use winit::dpi::PhysicalSize;
use wgpu::{CommandEncoderDescriptor, RenderPass, PipelineLayoutDescriptor};

// Main application listener
struct MyListener {
    renderer: Renderer
}

impl AppListener for MyListener {

    fn new(resources: &AppResources) -> Self {
        let device = resources.device;
        let config = RenderConfig { format: resources.config.format };
        Self { renderer: Renderer::new(device, &config) }
    }

    fn on_draw(&mut self, render_pass: &mut RenderPass) {

        //self.renderer().render(render_pass, &params);
    }

    fn on_resize(&mut self, size: PhysicalSize<u32>, resources: &AppResources) {
    }
}

fn main() {

    // Sets up logging
    env_logger::init();

    // Creates app and starts it
    let mut app = block_on(App::new());
    app.start::<MyListener>();
}