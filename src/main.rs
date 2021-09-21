use winit::window::{WindowBuilder, WindowId};
use winit::event::*;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use std::any::Any;
use tile_editor::State;
use pollster::block_on;

fn handle_key(input: KeyboardInput, control_flow: &mut ControlFlow) {
    if input.state == ElementState::Pressed && input.virtual_keycode == Some(VirtualKeyCode::Escape) {
        *control_flow = ControlFlow::Exit;
    }
}

fn handle_window_event(event: WindowEvent, control_flow: &mut ControlFlow) {
    match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput { input, .. } => handle_key(input, control_flow),
        _ => {}
    }
}

fn handle_suspend() {
    println!("Suspended");
}

fn handle_resume() {
    println!("Resuming");
}


fn main() {

    // Sets up logging
    env_logger::init();

    // Creates window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Map Editor")
        .build(&event_loop).unwrap();

    // Creates rendering state
    let state: State = block_on(State::new(&window));
    println!("Got state!");

    // Starts event loop and handles events
    event_loop.run(move |event, window_target, control_flow| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => { handle_window_event(event, control_flow) },
        Event::Suspended => { handle_suspend() },
        Event::Resumed => { handle_resume() },
        _ => {}
    });
}
