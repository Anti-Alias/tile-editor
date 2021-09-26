use winit::window::{WindowBuilder, WindowId};
use winit::event::*;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use std::any::Any;
use tile_editor::{App, Texture, AppListener, AppResources};
use pollster::block_on;
use log::info;

// Main application listener
struct MyListener {
    
}
impl AppListener for MyListener {
    fn on_start(&self, resources: &AppResources) {
        println!("App started");
    }
    fn on_draw(&self, resources: &AppResources) {
        println!("Drawing!");
    }
}

fn main() {

    // Sets up logging
    env_logger::init();

    // Creates app and starts it
    let listener = MyListener {};
    let mut app = block_on(App::new(listener));
    app.start();
}