use winit::window::{WindowBuilder, WindowId};
use winit::event::*;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use std::any::Any;
use tile_editor::{App, Texture};
use pollster::block_on;
use log::info;

async fn start() {

    // Sets up logging
    env_logger::init();

    // Creates app and consumes window
    let mut app = App::new().await;
    let device = app.device();
    let queue = app.queue();
    let tex = Texture::from_bytes(device, queue, include_bytes!("happy-tree.png"), None);

    // Starts/consumes app
    app.start();
}

fn main() {
    block_on(start());
}