use winit::window::{WindowBuilder, WindowId};
use winit::event::*;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use std::any::Any;
use tile_editor::{App, Texture, AppListener, AppResources};
use pollster::block_on;
use log::info;

struct MyListener {}
impl AppListener for MyListener {
    fn on_start(&self, resources: &AppResources) {
        println!("App started");
    }
    fn on_draw(&self, resources: &AppResources) {
        println!("Drawing!");
    }
}

async fn start() {

    // Sets up logging
    env_logger::init();

    // Creates app and consumes window
    let listener = MyListener {};
    let mut app = App::new(listener).await;
    let device = app.device();
    let queue = app.queue();
    let tex = Texture::from_bytes(device, queue, include_bytes!("happy-tree.png"), None);

    // Starts/consumes app
    app.start();
}

fn main() {
    block_on(start());
}