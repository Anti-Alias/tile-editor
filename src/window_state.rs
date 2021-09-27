use winit::window::Window;
use wgpu::{Surface, SurfaceConfiguration, Device};
use winit::dpi::PhysicalSize;
use winit::event::{WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use winit::event_loop::ControlFlow;
use log::info;


pub struct WindowState {
    pub window: Window,
    pub surface: Surface,
    pub config: SurfaceConfiguration
}

impl WindowState {

    /// Handles window event.
    /// Returns true if event was processed.
    ///
    /// # Arguments
    ///
    /// * `event` - Window event to consider
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        return false;
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn handle_window_event(
        &mut self,
        event: WindowEvent,
        device: &Device,
        control_flow: &mut ControlFlow
    ) {
        if !self.input(&event) {
            match event {
                WindowEvent::CloseRequested => Self::close(control_flow),
                WindowEvent::KeyboardInput { input, .. } => self.handle_key(input, control_flow),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self.resize(device, *new_inner_size),
                _ => {}
            }
        }
    }

    /// Resizes surface the new size specified
    pub fn resize(&mut self, device: &Device, new_size: PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(device, &self.config);
    }

    fn handle_key(&self, input: KeyboardInput, control_flow: &mut ControlFlow) {
        if input.state == ElementState::Pressed && input.virtual_keycode == Some(VirtualKeyCode::Escape) {
            Self::close(control_flow);
        }
    }

    fn handle_suspend(&self) {
        println!("Suspended");
    }

    fn handle_resume(&self) {
        println!("Resuming");
    }

    fn close(control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Exit;
        info!("See ya!");
    }
}